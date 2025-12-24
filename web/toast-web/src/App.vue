<script setup lang="ts">
import { ref } from 'vue';
import HelloWorld from './components/HelloWorld.vue'
import { getBaseURL } from './api';

const msg = ref('点击图标以关闭消息')

var flag = true
const switcher = () => {
  flag = !flag
  msg.value = flag ? '你应该可以看到消息了' : '你关闭了消息'
  fetch(`${getBaseURL()}/toast?msg=${flag ? '这是一个消息' : ''}&time=${flag ? 0 : 1}`)
}
</script>

<template>
  <div>
    <a @click="switcher" target="_blank">
      <img src="./assets/toast.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld :msg="msg" />
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
