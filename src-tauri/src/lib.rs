use tauri::{Emitter, Manager, State};
use tokio::time::{sleep, Duration};
use active_win_pos_rs::get_active_window;
use std::time::Instant;
use std::sync::Mutex;
use sysinfo::System;

// タイマー設定を保持する構造体
struct TimerConfig {
    target_end_time: Mutex<Option<Instant>>,
    remaining_secs: Mutex<i64>, // STOP時の残り時間を保持
    is_running: Mutex<bool>,
}

#[tauri::command]
fn set_timer_by_minutes(state: State<TimerConfig>, minutes: u64) {
    let total_secs = (minutes * 60) as i64;
    let mut target = state.target_end_time.lock().unwrap();
    let mut rem = state.remaining_secs.lock().unwrap();
    let mut running = state.is_running.lock().unwrap();

    *rem = total_secs;
    *target = Some(Instant::now() + Duration::from_secs(minutes * 60));
    *running = true;
    println!("[DEBUG] タイマーを手動設定: {}分間", minutes);
}

#[tauri::command]
fn set_timer_by_mmss(state: State<TimerConfig>, minutes: u64, seconds: u64) {
    let total_seconds = minutes * 60 + seconds;
    if total_seconds > 0 {
        let mut target = state.target_end_time.lock().unwrap();
        let mut rem = state.remaining_secs.lock().unwrap();
        let mut running = state.is_running.lock().unwrap();

        *rem = total_seconds as i64;
        *target = Some(Instant::now() + Duration::from_secs(total_seconds));
        *running = true;
        println!("[DEBUG] 残り時間を設定: {}分{}秒 (計 {} 秒)", minutes, seconds, total_seconds);
    } else {
        println!("[DEBUG] [ERROR] 有効な時間が設定されていません。");
    }
}

// タイマーの開始・再開 (START)
#[tauri::command]
fn start_timer(state: State<TimerConfig>) {
    let mut target = state.target_end_time.lock().unwrap();
    let rem = state.remaining_secs.lock().unwrap();
    let mut running = state.is_running.lock().unwrap();

    if !*running {
        let secs_to_run = if *rem > 0 { *rem as u64 } else { 300 }; // 0秒以下ならデフォルト5分
        *target = Some(Instant::now() + Duration::from_secs(secs_to_run));
        *running = true;
        println!("[DEBUG] タイマーをスタート/再開しました (残り: {}秒)", secs_to_run);
    }
}

// タイマーの一時停止 (STOP)
#[tauri::command]
fn stop_timer(state: State<TimerConfig>) {
    let mut target = state.target_end_time.lock().unwrap();
    let mut rem = state.remaining_secs.lock().unwrap();
    let mut running = state.is_running.lock().unwrap();

    if *running {
        if let Some(end_instant) = *target {
            let now = Instant::now();
            *rem = if end_instant > now {
                (end_instant - now).as_secs() as i64
            } else {
                0
            };
        }
        *target = None;
        *running = false;
        println!("[DEBUG] タイマーを一時停止しました (保持時間: {}秒)", *rem);
    }
}

// アプリを完全に終了するコマンド
#[tauri::command]
fn close_app(app: tauri::AppHandle) {
    println!("[DEBUG] アプリケーションを終了します。");
    app.exit(0);
}

fn start_window_monitor(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut was_in_meeting = false;

        // システム情報の初期化
        let mut sys = System::new_all();

        loop {
            // CPUおよびメモリ情報の更新
            sys.refresh_cpu();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let total_mem = sys.total_memory();
            let used_mem = sys.used_memory();
            let memory_usage = if total_mem > 0 {
                (used_mem as f32 / total_mem as f32) * 100.0
            } else {
                0.0
            };

            // バッテリー取得処理
            let raw_power_watts: f32 = {
                let mut watts = 0.0;
                if let Ok(manager) = battery::Manager::new() {
                    if let Ok(batteries) = manager.batteries() {
                        for battery in batteries {
                            if let Ok(b) = battery {
                                let rate = b.energy_rate();
                                watts = rate.value;
                            }
                        }
                    }
                }
                watts
            };

            // 電源接続時（0W表示時）は CPU 使用率に基づく概算消費電力（W）に切り替え
            let displayed_watts = if raw_power_watts > 0.1 {
                raw_power_watts
            } else {
                10.0 + (cpu_usage / 100.0) * 35.0
            };

            let mut is_in_meeting = false;
            let mut detected_title = String::new();

            match get_active_window() {
                Ok(window) => {
                    let title = window.title.to_lowercase();
                    detected_title = window.title.clone();
                    let is_google_meet = title.contains("google meet");
                    let is_teams = title.contains("microsoft teams");

                    if is_google_meet || is_teams {
                        is_in_meeting = true;
                    }
                }
                Err(_) => {}
            }

            if is_in_meeting && !was_in_meeting {
                println!("[DEBUG] ★ 会議を検出しました！ ➔ タイトル: {}", detected_title);
            } else if !is_in_meeting && was_in_meeting {
                println!("[DEBUG] 会議が終了（または非検知）になりました。");
            }
            was_in_meeting = is_in_meeting;

            let default_duration = Duration::from_secs(25 * 60);
            let config = app.state::<TimerConfig>();

            let (is_running_val, remaining_seconds) = {
                let mut running_lock = config.is_running.lock().unwrap();
                let mut target_lock = config.target_end_time.lock().unwrap();
                let mut rem_lock = config.remaining_secs.lock().unwrap();

                // 会議自動検知時（停止中の場合のみ自動スタート）
                if is_in_meeting && !*running_lock {
                    *running_lock = true;
                    *rem_lock = default_duration.as_secs() as i64;
                    *target_lock = Some(Instant::now() + default_duration);
                    println!("[DEBUG] ★ 会議を検出しました！タイマーを自動スタート（25分） ➔ タイトル: {}", detected_title);
                }

                if *running_lock {
                    if let Some(end_instant) = *target_lock {
                        let now = Instant::now();
                        let rem = if end_instant > now {
                            (end_instant - now).as_secs() as i64
                        } else {
                            0
                        };
                        *rem_lock = rem;
                        if rem <= 0 {
                            *running_lock = false;
                        }
                        (true, rem)
                    } else {
                        (false, *rem_lock)
                    }
                } else {
                    (false, *rem_lock)
                }
            };

            let is_critical = is_running_val && remaining_seconds <= 180 && remaining_seconds > 0;

            // フロントエンドへイベント送信（isRunningを追加）
            let _ = app.emit("meeting-status", serde_json::json!({
                "isInMeeting": is_in_meeting,
                "remainingSeconds": remaining_seconds,
                "isCritical": is_critical,
                "isRunning": is_running_val,
                "powerWatts": (((displayed_watts * 10.0) as f32).round() / 10.0).floor(),
                "cpuUsage": cpu_usage,
                "memoryUsage": memory_usage
            }));

            sleep(Duration::from_secs(1)).await;
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(TimerConfig {
            target_end_time: Mutex::new(None),
            remaining_secs: Mutex::new(300), // 初期表示用 (5分)
            is_running: Mutex::new(false),
        })
        .setup(move |app| {
            let handle = app.handle().clone();
            start_window_monitor(handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_timer_by_minutes, 
            set_timer_by_mmss, 
            start_timer,
            stop_timer,
            close_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}