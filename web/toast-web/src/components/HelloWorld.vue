<script setup lang="ts">
import { ref } from 'vue'
import { getBaseURL } from '../api';

defineProps<{ msg: string }>()

const count = ref(0)
const alpha = ref(190)

const set_alpha = () => {
  fetch(`${getBaseURL()}/setalpha?alpha=${alpha.value}`)
}

const set_pos = () => {
  fetch(`${getBaseURL()}/toast?msg=挪动鼠标后单击确定`)
  if (document.fullscreenEnabled) {
    document.documentElement.requestFullscreen()
  }
  const fn = (event: MouseEvent) => {
    fetch(`${getBaseURL()}/setpos?x=${event.screenX}&y=${event.screenY}`)
  }
  document.addEventListener('mousemove', fn)
  document.addEventListener('mousedown', () => {
    document.removeEventListener('mousemove', fn)
    document.exitFullscreen()
    fetch(`${getBaseURL()}/toast?msg=设置完毕`)
  }, { once: true })
}

const test = () => {
  count.value++
  fetch(`${getBaseURL()}/toast?msg=${`消息内容: ${count.value}`}`)
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <input v-model="alpha" @input="set_alpha" type="range" id="slider" name="slider" min="0" max="255" value="190" step="1">
  <div>
    不透明度: {{ alpha }}
  </div>
  <div class="card">
    <button type="button" @click="test">{{ count == 0 ? '测试消息' : `count is ${count}` }}</button>
    <button type="button" @click="set_pos">修改toast位置</button>
  </div>

  <p>
    API
  </p>
  <p class="read-the-docs">
    GET {{ getBaseURL() }}/toast?msg=123456&time=2000 (设置消息与显示时长[毫秒], 0为永久显示)
  </p>
  <p class="read-the-docs">
    GET {{ getBaseURL() }}/setpos?x=1000&y=550 (设置窗口位置)
  </p>
  <p class="read-the-docs">
    GET {{ getBaseURL() }}/setwh?w=860&h=200 (设置窗口宽高)
  </p>
  <p class="read-the-docs">
    GET {{ getBaseURL() }}/setalpha?alpha=190 (设置窗口整体不透明度, 0为完全透明, 255为完全不透明)
  </p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}

input[type="range"] {
  /* 移除默认样式 */
  appearance: none;
  width: 300px;
  height: 10px;
  background: #ddd;
  border-radius: 5px;
  outline: none;
}

input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  width: 20px;
  height: 20px;
  background: #4CAF50;
  border-radius: 50%;
  cursor: pointer;
}

input[type="range"]::-moz-range-thumb {
  width: 20px;
  height: 20px;
  background: #4CAF50;
  border-radius: 50%;
  cursor: pointer;
  border: none;
}
</style>