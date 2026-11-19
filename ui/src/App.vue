<template>
  <div class="min-h-screen bg-[#f7f8fc] text-[#172033]">
    <header class="flex h-[72px] items-center justify-between gap-8 border-b border-[#edf0f5] bg-white px-[7%] max-sm:h-auto max-sm:min-h-[68px] max-sm:flex-wrap max-sm:gap-2 max-sm:px-[5%]">
      <div class="text-xl font-extrabold -tracking-[0.5px] text-[#5965e8]">共享打印服务</div>
      <nav class="flex h-full items-center gap-2 max-sm:order-3 max-sm:h-12 max-sm:w-full max-sm:justify-between" aria-label="页面导航">
        <button v-for="item in tabs" :key="item"
          class="relative h-full px-[18px] text-sm text-[#697386] transition-colors hover:text-[#5965e8] max-sm:px-[10px]"
          :class="activeTab === item ? 'font-semibold text-[#5965e8] after:absolute after:right-[18px] after:bottom-0 after:left-[18px] after:h-[3px] after:rounded-t-sm after:bg-[#5965e8] max-sm:after:right-[10px] max-sm:after:left-[10px]' : ''"
          :aria-current="activeTab === item ? 'page' : undefined" @click="activeTab = item">
          {{ item }}
        </button>
      </nav>
      <button class="grid h-9 w-9 place-items-center rounded-full bg-[#eef0ff] font-bold text-[#5965e8]" aria-label="个人中心">A</button>
    </header>

    <main class="mx-auto my-24 w-[min(880px,86%)] max-sm:my-16">
      <PdfPreview v-if="activeTab === '首页'" />
      <History v-else-if="activeTab === '历史记录'" />
      <PrinterStatus v-else-if="activeTab === '打印机状态管理'" />
      <AboutUs v-else-if="activeTab === '关于我们'" />
      <template v-else>
        <p class="mb-3 text-[13px] font-semibold tracking-[1px] text-[#5965e8]">欢迎使用</p>
        <h1 class="m-0 text-[clamp(36px,6vw,52px)] font-bold -tracking-[2px]">{{ activeTab }}</h1>
        <p class="mt-[18px] mb-10 leading-[1.8] text-[#788196]">这是一个简洁的头部切换栏示例，点击上方导航切换页面内容。</p>
        <div class="flex items-center gap-[18px] rounded-[14px] border border-[#edf0f5] bg-white p-[26px] shadow-[0_10px_30px_rgba(34,45,80,0.04)]">
          <span class="grid h-[46px] w-[46px] place-items-center rounded-xl bg-[#f0f1ff] text-[22px] text-[#5965e8]">✦</span>
          <div>
            <strong class="text-[15px]">{{ activeTab }}页面</strong>
            <p class="mt-[7px] mb-0 text-[13px] text-[#8991a2]">当前已切换至「{{ activeTab }}」</p>
          </div>
        </div>
      </template>
    </main>
    <!-- 底部版权信息 -->
    <footer class="absolute bottom-4 w-full text-center text-xs text-gray-400 z-10">
      © 2025 打印助手 · 安全、高效的在线打印解决方案
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import PdfPreview from './components/PdfPreview.vue'
import History from './components/History.vue'
import PrinterStatus from './components/PrinterStatus.vue'
import AboutUs from './components/AboutUs.vue'

const tabs = ['首页', '历史记录', '打印机状态管理', '关于我们']
const activeTab = ref(tabs[0])
</script>
