<template>
  <div 
    :class="['eva-container', { 'critical-mode': isCritical, 'finished-mode': isFinished }]" 
    data-tauri-drag-region
    @contextmenu.prevent="toggleMenu"
  >
    <!-- 通常・警告モードの表示 -->
    <template v-if="!isFinished">
      <!-- 上部ヘッダー（設定可能なタイトル & ハザードストライプ） -->
      <div class="top-header-bar" data-tauri-drag-region>
        <div class="header-title-wrapper" ref="headerTitleWrapper" data-tauri-drag-region>
          <div 
            class="header-title" 
            ref="headerTitleText"
            :style="{ fontSize: titleFontSize + 'px' }"
            data-tauri-drag-region
          >
            {{ headerTitle }}
          </div>
        </div>
        <div class="hazard-stripes" data-tauri-drag-region></div>
      </div>

      <!-- 画面右上の常時閉じるボタン（×） -->
      <button class="top-close-btn" @mousedown.stop @click.stop="closeApp" title="アプリを終了">[ × ]</button>

      <!-- 中央タイマー表示 -->
      <div class="monitor-content" ref="monitorContentRef" data-tauri-drag-region>
        <div 
          class="timer-display" 
          ref="timerDisplayRef"
          :style="{ fontSize: timerFontSize + 'px' }"
          data-tauri-drag-region
        >
          {{ formatTime(remainingSeconds) }}
        </div>
      </div>

      <!-- 下部ステータスパネル（主電源供給システム / CPU・メモリ使用率） -->
      <div class="bottom-panel" data-tauri-drag-region>
        <div class="panel-box warning-box" data-tauri-drag-region>
          <div class="box-main" data-tauri-drag-region>主電源供給 <span class="ext-value" data-tauri-drag-region>{{ powerWatts }} <span style="font-family: 'TsukuhouShogoMin-OFL'">W</span></span></div>
          <div class="box-sub" data-tauri-drag-region>MAINENERGY SUPPLY SYSTEM </div>
        </div>
        
        <!-- CPU・メモリ表示 -->
        <div class="panel-box external-box" data-tauri-drag-region>
          <div class="sys-metrics-row" data-tauri-drag-region>
            <span class="metric-item" data-tauri-drag-region>CPU <span class="val" data-tauri-drag-region>{{ cpuUsage }}%</span></span>
            <span class="metric-item" data-tauri-drag-region>MEM <span class="val" data-tauri-drag-region>{{ memoryUsage }}%</span></span>
          </div>
          <div class="box-sub" data-tauri-drag-region>SYSTEM RESOURCE</div>
        </div>
      </div>

      <!-- 下部ハザードストライプ -->
      <div class="bottom-hazard-bar" data-tauri-drag-region></div>
    </template>

    <!-- 終了モード（残り0秒）の表示 -->
    <template v-else>
      <!-- 画面右上の常時閉じるボタン（×） -->
      <button class="top-close-btn finished-close-btn" @mousedown.stop @click.stop="closeApp" title="アプリを終了">[ × ]</button>

      <div class="finished-layout" data-tauri-drag-region>
        <!-- 上部ヘッダー -->
        <div class="fin-header" data-tauri-drag-region>
          <span class="fin-title-jp">{{ headerTitle }} :</span>
        </div>

        <div class="fin-body" data-tauri-drag-region>
          <!-- 中央：ユーザー設定可能な終了文字 -->
          <div class="fin-center-text-wrapper" data-tauri-drag-region>
            <div class="fin-center-text" data-tauri-drag-region>
              {{ finishedText }}
            </div>
          </div>

          <!-- 右側パネル群（内部 / 主電源供給システム / 外部） -->
          <div class="fin-right-panels" data-tauri-drag-region>
            <div class="fin-panel-item" data-tauri-drag-region>
              <div class="fin-panel-main">内部</div>
              <div class="fin-panel-sub">INTERNAL</div>
            </div>
            <div class="fin-panel-sys" data-tauri-drag-region>
              主電源供給システム<br>MAIN ENERGY SUPPLY SYSTEM
            </div>
            <div class="fin-panel-item" data-tauri-drag-region>
              <div class="fin-panel-main">外部</div>
              <div class="fin-panel-sub">EXTERNAL</div>
            </div>
          </div>
        </div>

        <!-- 下部フッター -->
        <div class="fin-footer" data-tauri-drag-region>
          <div class="fin-status-boxes" data-tauri-drag-region>
            <div class="fin-sbox active-sbox">STOP</div>
            <div class="fin-sbox">SLOW</div>
            <div class="fin-sbox">NORMAL</div>
            <div class="fin-sbox">RACING</div>
          </div>
          <div class="fin-danger-box" data-tauri-drag-region>
            DANGER<br>EMERGENCY
          </div>
        </div>
      </div>
    </template>

    <!-- 右クリック設定メニュー -->
    <div v-if="showMenu" class="eva-menu" @mousedown.stop>
      <div class="menu-title">-- TIMER CONFIG --</div>
      
      <!-- タイマー時間入力 -->
      <div class="time-input-section">
        <span>残り:</span>
        <input type="number" v-model.number="inputMinutes" min="0" placeholder="1" class="eva-input-num" @mousedown.stop />
        <span class="input-unit">m</span>
        <input type="number" v-model.number="inputSeconds" min="0" max="59" placeholder="1" class="eva-input-num" @mousedown.stop />
        <span class="input-unit">s</span>
        <button @click="setCustomTime" class="set-btn">SET</button>
      </div>

      <!-- タイトル変更用入力欄 -->
      <div class="time-input-section">
        <span>タイトル:</span>
        <input type="text" v-model="headerTitle" class="eva-input-text" placeholder="活動限界まで ACTIVE TIME REMAINING" @mousedown.stop />
      </div>

      <!-- 終了時の文字変更用入力欄 -->
      <div class="time-input-section">
        <span>終了文字:</span>
        <input type="text" v-model="finishedText" class="eva-input-text" placeholder="活動終了" @mousedown.stop />
      </div>

      <!-- STOP / START ボタン -->
      <button @click="toggleStartStop" class="stop-btn">[ STOP / START ]</button>
      <button @click="showMenu = false" class="close-btn">[ CLOSE MENU ]</button>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';

const remainingSeconds = ref(300);
const isCritical = ref(false);
const isInMeeting = ref(false);
const isFinished = ref(false);
const showMenu = ref(false);
const isRunning = ref(true);

const inputMinutes = ref(1);
const inputSeconds = ref(1);

// タイトル表示テキスト用のステート
const headerTitle = ref('活動限界まで ACTIVE TIME REMAINING');
const finishedText = ref('活動終了');

// 動的フォントサイズ管理用のRef (タイトル用)
const headerTitleWrapper = ref(null);
const headerTitleText = ref(null);
const titleFontSize = ref(13);

// 動的フォントサイズ管理用のRef (タイマー数字用)
const monitorContentRef = ref(null);
const timerDisplayRef = ref(null);
const timerFontSize = ref(80);

// システム情報のリアルタイム表示用ステート
const powerWatts = ref(0);
const cpuUsage = ref(0);
const memoryUsage = ref(0);

const appWindow = getCurrentWindow();

// ヘッダータイトルのフォントサイズを枠幅に合わせて最大化する関数
const adjustTitleFontSize = async () => {
  await nextTick();
  if (!headerTitleWrapper.value || !headerTitleText.value) return;

  const wrapperWidth = headerTitleWrapper.value.clientWidth;
  if (wrapperWidth <= 0) return;

  let size = 20; 
  headerTitleText.value.style.fontSize = `${size}px`;

  while (headerTitleText.value.scrollWidth > wrapperWidth && size > 6) {
    size -= 0.5;
    headerTitleText.value.style.fontSize = `${size}px`;
  }

  titleFontSize.value = size;
};

// タイマー数字のフォントサイズを余白を持たせて動的に調整する関数
const adjustTimerFontSize = async () => {
  await nextTick();
  if (!monitorContentRef.value || !timerDisplayRef.value) return;

  const paddingRatio = 0.80;

  const containerWidth = monitorContentRef.value.clientWidth * paddingRatio;
  const containerHeight = monitorContentRef.value.clientHeight * paddingRatio;
  if (containerWidth <= 0 || containerHeight <= 0) return;

  let size = Math.floor(containerHeight);
  if (size < 12) size = 12;

  timerDisplayRef.value.style.fontSize = `${size}px`;

  while (
    (timerDisplayRef.value.scrollWidth > containerWidth || 
     timerDisplayRef.value.scrollHeight > containerHeight) && 
    size > 12
  ) {
    size -= 1;
    timerDisplayRef.value.style.fontSize = `${size}px`;
  }

  timerFontSize.value = size;
};

let resizeObserver = null;

watch(headerTitle, () => {
  adjustTitleFontSize();
});

watch(remainingSeconds, () => {
  adjustTimerFontSize();
});

watch([isCritical, isFinished], async ([critical, finished]) => {
  const shouldTop = critical || finished;
  try {
    await appWindow.setAlwaysOnTop(shouldTop);
  } catch (err) {
    console.error("Failed to set always on top:", err);
  }
}, { immediate: true });

const toggleMenu = () => {
  showMenu.value = !showMenu.value;
};

const setCustomTime = async () => {
  const mins = Number(inputMinutes.value) || 0;
  const secs = Number(inputSeconds.value) || 0;

  if (mins === 0 && secs === 0) {
    alert("時間が設定されていません。");
    return;
  }

  isFinished.value = false;
  try {
    await invoke('set_timer_by_mmss', { minutes: mins, seconds: secs });
    showMenu.value = false;
  } catch (err) {
    console.error("Failed to set custom time:", err);
    alert("設定に失敗しました: " + err);
  }
};

const toggleStartStop = async () => {
  isFinished.value = false;
  try {
    if (isRunning.value) {
      await invoke('stop_timer');
    } else {
      await invoke('start_timer').catch(async () => {
        await invoke('toggle_timer');
      });
    }
    showMenu.value = false;
  } catch (err) {
    console.error("Failed to toggle timer:", err);
  }
};

const closeApp = async () => {
  try {
    await invoke('close_app');
  } catch (err) {
    console.error("Failed to close app:", err);
  }
};

const formatTime = (sec) => {
  if (sec < 0) sec = 0;
  const m = Math.floor(sec / 60).toString();
  const s = (sec % 60).toString().padStart(2, '0');
  return `${m}:${s}`;
};

onMounted(async () => {
  resizeObserver = new ResizeObserver(() => {
    adjustTitleFontSize();
    adjustTimerFontSize();
  });

  if (headerTitleWrapper.value) {
    resizeObserver.observe(headerTitleWrapper.value);
  }
  if (monitorContentRef.value) {
    resizeObserver.observe(monitorContentRef.value);
  }

  adjustTitleFontSize();
  adjustTimerFontSize();

  await listen('meeting-status', (event) => {
    remainingSeconds.value = event.payload.remainingSeconds;
    isCritical.value = event.payload.isCritical;
    isInMeeting.value = event.payload.isInMeeting;

    if (event.payload.isRunning !== undefined) {
      isRunning.value = event.payload.isRunning;
    }

    if (event.payload.powerWatts !== undefined) powerWatts.value = event.payload.powerWatts;
    if (event.payload.cpuUsage !== undefined) cpuUsage.value = Math.round(event.payload.cpuUsage);
    if (event.payload.memoryUsage !== undefined) memoryUsage.value = Math.round(event.payload.memoryUsage);

    if (remainingSeconds.value <= 0) {
      isFinished.value = true;
    }
  });
});

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect();
  }
});
</script>

<style>
html, body {
  margin: 0;
  padding: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  background: #000;
}
</style>

<style scoped>
@import url('https://cdn.jsdelivr.net/npm/dseg@0.46.0/css/dseg.css');

@font-face {
  font-family: 'EvaCustomFont';
  src: url('./assets/TsukuhouShogoMin-OFL.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

.eva-container {
  width: 100vw;
  height: 100vh;
  background: rgba(0, 20, 0, 0.9);
  border: 3px solid #00ff66;
  color: #00ff66;
  font-family: 'EvaCustomFont', sans-serif;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: stretch;
  user-select: none;
  cursor: grab;
  position: relative;
  overflow: hidden;
  box-sizing: border-box;
  padding: 4px;
}

.eva-container:active {
  cursor: grabbing;
}

.eva-container.critical-mode {
  background: rgba(30, 0, 0, 0.95);
  border-color: #ff0033;
  color: #ff0033;
  animation: bg-flash 1s infinite alternate;
}

/* 終了モード（背景＆ウィンドウ枠全体の点滅） */
.eva-container.finished-mode {
  background: #1a0000;
  border-color: #ff0033;
  color: #ff0033;
  animation: fin-bg-flash 0.8s infinite alternate;
}

@keyframes bg-flash {
  0% { background: rgba(30, 0, 0, 0.9); }
  100% { background: rgba(80, 0, 0, 0.95); }
}

/* 終了モード用の点滅キーフレーム定義 */
@keyframes fin-bg-flash {
  0% { 
    background: #100000; 
    border-color: #ff0033; 
    box-shadow: inset 0 0 10px rgba(255, 0, 51, 0.3);
  }
  100% { 
    background: #3a0005; 
    border-color: #ff6688; 
    box-shadow: inset 0 0 25px rgba(255, 0, 51, 0.8);
  }
}

@keyframes fin-text-pulse {
  0%, 100% { 
    opacity: 1; 
    filter: drop-shadow(0 0 8px #ff0033);
  }
  50% { 
    opacity: 0.2; 
    filter: drop-shadow(0 0 1px #ff0033);
  }
}

@keyframes fin-box-invert {
  0%, 49% { 
    background: #ff0033; 
    color: #000000; 
  }
  50%, 100% { 
    background: #000000; 
    color: #ff0033; 
  }
}

@keyframes fin-danger-flash {
  0%, 100% { 
    background: #000000; 
    color: #ff0033; 
    border-color: #ff0033;
  }
  50% { 
    background: #ff0033; 
    color: #000000; 
    border-color: #ffffff;
  }
}

@keyframes fin-panel-border-flash {
  0%, 100% { border-color: #ff0033; }
  50% { border-color: #ff88a0; }
}

.top-header-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 24px;
  background: #000;
  border-bottom: 2px solid currentColor;
  padding: 0 4px;
  padding-right: 28px;
  gap: 8px;
  overflow: hidden;
}

.header-title-wrapper {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  overflow: hidden;
  height: 100%;
}

.header-title {
  font-weight: 900;
  letter-spacing: 0.5px;
  white-space: nowrap;
  line-height: 1;
  display: inline-block;
}

.hazard-stripes, .bottom-hazard-bar {
  height: 10px;
  width: 60px;
  flex-shrink: 0;
  background: repeating-linear-gradient(
    -45deg,
    currentColor,
    currentColor 5px,
    #000 5px,
    #000 10px
  );
}

.bottom-hazard-bar {
  width: 100%;
  height: 8px;
  background: repeating-linear-gradient(
    -45deg,
    currentColor,
    currentColor 6px,
    #000 6px,
    #000 12px
  );
}

.top-close-btn {
  position: absolute;
  top: 4px;
  right: 4px;
  background: #000;
  border: 1px solid currentColor;
  color: inherit;
  font-family: inherit;
  font-size: 9px;
  padding: 1px 4px;
  cursor: pointer;
  z-index: 50;
}

.top-close-btn:hover {
  background: rgba(0, 255, 102, 0.3);
}

.critical-mode .top-close-btn:hover, .finished-mode .top-close-btn:hover {
  background: rgba(255, 0, 51, 0.3);
}

.monitor-content {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
  min-height: 0;
}

/* タイマー数値表示 */
.timer-display {
  font-family: 'DSEG7-Classic', 'EvaCustomFont', monospace;
  font-weight: bold;
  letter-spacing: 2px;
  line-height: 1;
  text-shadow: 0 0 8px currentColor;
  white-space: nowrap;
  display: inline-block;
}

.bottom-panel {
  display: flex;
  gap: 6px;
  margin-bottom: 4px;
}

.panel-box {
  flex: 1;
  border: 2px solid currentColor;
  padding: 3px 6px;
  background: #000;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.box-main {
  font-size: 13px;
  font-weight: bold;
  letter-spacing: 0.5px;
}

.box-sub {
  font-size: 7px;
  letter-spacing: 1px;
  opacity: 0.8;
}

.ext-value {
  font-family: 'DSEG7-Classic', 'EvaCustomFont', monospace;
  font-size: 13px;
  float: right;
  text-shadow: 0 0 4px currentColor;
}

.sys-metrics-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  font-weight: bold;
  letter-spacing: 0.5px;
}

.metric-item {
  display: flex;
  gap: 4px;
  align-items: center;
}

.val {
  font-family: 'DSEG7-Classic', 'EvaCustomFont', monospace;
  font-size: 13px;
  text-shadow: 0 0 4px currentColor;
}

.finished-layout {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  box-sizing: border-box;
  padding: 1px;
  overflow: hidden;
}

.fin-header {
  display: flex;
  align-items: baseline;
  gap: 8px;
  background: #000;
  border-bottom: 2px solid #ff0033;
  padding: 1px 4px;
  white-space: nowrap;
  animation: fin-panel-border-flash 0.6s infinite alternate;
}

.fin-title-jp {
  font-size: 12px;
  font-weight: bold;
  letter-spacing: 1px;
  animation: fin-text-pulse infinite steps(1);
}

.fin-body {
  flex: 1;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0;
  margin: 0;
  gap: 2px;
  overflow: hidden;
}

.fin-center-text-wrapper {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0;
  margin: 0;
  overflow: visible;
  height: 100%;
}

/* 中央テキスト「活動終了」の強烈な点滅効果 */
.fin-center-text {
  font-size: 64px;
  font-weight: 900;
  letter-spacing: 1px;
  white-space: nowrap;
  transform: scaleX(0.92) scaleY(1.55);
  transform-origin: center;
  line-height: 1;
  text-align: center;
}

.fin-right-panels {
  display: flex;
  flex-direction: column;
  gap: 2px;
  width: 105px;
  flex-shrink: 0;
}

.fin-panel-item {
  border: 1.5px solid #ff0033;
  background: #000;
  padding: 2px 4px;
  position: relative;
  background-image: repeating-linear-gradient(
    -45deg,
    transparent,
    transparent 5px,
    rgba(255, 0, 51, 0.15) 5px,
    rgba(255, 0, 51, 0.15) 10px
  );
  animation: fin-panel-border-flash 0.8s infinite alternate;
}

.fin-panel-main {
  font-size: 11px;
  font-weight: bold;
}

.fin-panel-sub {
  font-size: 6px;
  letter-spacing: 0.5px;
}

.fin-panel-sys {
  border: 1px solid #ff0033;
  background: #000;
  font-size: 5px;
  text-align: center;
  padding: 1px;
  line-height: 1.1;
  letter-spacing: 0.1px;
  animation: fin-panel-border-flash 0.5s infinite alternate;
}

.fin-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #000;
  border: 2px solid #ff0033;
  padding: 2px;
  box-sizing: border-box;
  animation: fin-panel-border-flash 0.6s infinite alternate;
}

.fin-status-boxes {
  display: flex;
  gap: 2px;
}

.fin-sbox {
  border: 1px solid #ff0033;
  padding: 1px 3px;
  font-size: 8px;
  font-weight: bold;
  background: #000;
}

/* 「STOP」ラベルの黒赤反転点滅 */
.active-sbox {
  animation: fin-box-invert 0.5s infinite steps(1);
}

/* 右下「DANGER / EMERGENCY」の交互点滅 */
.fin-danger-box {
  border: 1px solid #ff0033;
  padding: 1px 4px;
  font-size: 6px;
  font-weight: bold;
  text-align: center;
  line-height: 1.1;
  animation: fin-danger-flash 0.4s infinite alternate;
}

.eva-menu {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: rgba(0, 10, 0, 0.95);
  border: 2px solid currentColor;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  z-index: 100;
  box-shadow: 0 0 15px rgba(0, 255, 102, 0.2);
  min-width: 240px;
}

.critical-mode .eva-menu, .finished-mode .eva-menu {
  background: rgba(10, 0, 0, 0.95);
  box-shadow: 0 0 15px rgba(255, 0, 51, 0.2);
}

.menu-title {
  font-size: 11px;
  letter-spacing: 1px;
  text-align: center;
  font-weight: bold;
}

.set-btn, .stop-btn, .close-btn {
  background: transparent;
  border: 1px solid currentColor;
  color: inherit;
  padding: 4px 8px;
  font-family: inherit;
  font-size: 11px;
  cursor: pointer;
  transition: background 0.2s;
}

.set-btn:hover, .stop-btn:hover, .close-btn:hover {
  background: rgba(0, 255, 102, 0.2);
}

.critical-mode .set-btn:hover, .critical-mode .stop-btn:hover, .critical-mode .close-btn:hover,
.finished-mode .set-btn:hover, .finished-mode .stop-btn:hover, .finished-mode .close-btn:hover {
  background: rgba(255, 0, 51, 0.2);
}

.time-input-section {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
}

.eva-input-num {
  background: #000;
  border: 1px solid currentColor;
  color: inherit;
  padding: 2px 4px;
  font-family: inherit;
  font-size: 11px;
  width: 40px;
  text-align: center;
}

.eva-input-text {
  background: #000;
  border: 1px solid currentColor;
  color: inherit;
  padding: 2px 6px;
  font-family: inherit;
  font-size: 11px;
  flex: 1;
}

.eva-input-num::-webkit-inner-spin-button,
.eva-input-num::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.input-unit {
  font-weight: bold;
  margin-right: 4px;
}

.stop-btn, .close-btn {
  width: 100%;
}
</style>