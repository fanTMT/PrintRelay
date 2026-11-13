<template>
  <div class="min-h-screen bg-[#f5f6f8] text-gray-800 font-sans relative overflow-hidden">
    <!-- 背景装饰 (模拟截图右侧的模糊效果) -->
    <div class="absolute right-0 top-20 w-96 h-96 bg-white/40 rounded-full blur-3xl pointer-events-none"></div>
    <div class="absolute left-0 bottom-0 w-96 h-96 bg-white/40 rounded-full blur-3xl pointer-events-none"></div>

    <!-- 主内容区 -->
    <main class="max-w-7xl mx-auto px-6 py-8 relative z-10">
      
      <!-- 页面标题 -->
      <h1 class="text-2xl font-bold text-gray-900 mb-6">打印历史记录</h1>

      <!-- 数据统计卡片区 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        
        <!-- 卡片 1 -->
        <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100 flex flex-col justify-between relative">
          <div class="flex justify-between items-center mb-4">
            <span class="text-sm text-gray-500">总打印次数</span>
            <div class="w-3 h-3 bg-purple-200 rounded-sm"></div>
          </div>
          <div>
            <div class="text-3xl font-bold text-gray-900 mb-1">1</div>
            <div class="text-xs text-gray-500">本月: 0 次</div>
          </div>
        </div>

        <!-- 卡片 2 -->
        <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100 flex flex-col justify-between relative">
          <div class="flex justify-between items-center mb-4">
            <span class="text-sm text-gray-500">已完成打印</span>
            <div class="w-3 h-3 bg-green-200 rounded-sm"></div>
          </div>
          <div>
            <div class="text-3xl font-bold text-gray-900 mb-1">1</div>
            <div class="text-xs text-gray-500">成功率: 100%</div>
          </div>
        </div>

        <!-- 卡片 3 -->
        <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-100 flex flex-col justify-between relative">
          <div class="flex justify-between items-center mb-4">
            <span class="text-sm text-gray-500">打印文件大小</span>
            <div class="w-3 h-3 bg-blue-200 rounded-sm"></div>
          </div>
          <div>
            <div class="text-3xl font-bold text-gray-900 mb-1">207.6 KB</div>
            <div class="text-xs text-gray-500">平均: 207.6 KB</div>
          </div>
        </div>

      </div>

      <!-- 历史记录列表区域 -->
      <div class="bg-white rounded-xl shadow-sm border border-gray-100 p-6">
        
        <!-- 列表头部工具条 -->
        <div class="flex flex-col md:flex-row justify-between items-center mb-6 space-y-4 md:space-y-0">
          <h2 class="text-lg font-bold text-gray-900 w-full md:w-auto">历史记录列表</h2>
          
          <div class="flex items-center space-x-3 w-full md:w-auto justify-end">
            <!-- 搜索框 -->
            <div class="relative">
              <input 
                type="text" 
                placeholder="搜索文件名..." 
                class="pl-3 pr-4 py-1.5 border border-gray-200 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 w-64 text-gray-600"
              >
            </div>
            <!-- 筛选按钮 -->
            <button class="px-4 py-1.5 border border-gray-200 rounded-md text-sm text-gray-600 hover:bg-gray-50 transition-colors">
              筛选
            </button>
            <!-- 清空按钮 -->
            <button class="px-4 py-1.5 bg-[#ea5455] text-white rounded-md text-sm hover:bg-red-500 transition-colors shadow-sm">
              清空历史
            </button>
          </div>
        </div>

        <!-- 表格 -->
        <div class="overflow-x-auto">
          <table class="w-full text-left border-collapse">
            <thead>
              <tr class="border-b border-gray-100 text-sm text-gray-500">
                <th class="py-4 px-4 font-normal w-16">ID</th>
                <th class="py-4 px-4 font-normal">文件名</th>
                <th class="py-4 px-4 font-normal">文件大小</th>
                <th class="py-4 px-4 font-normal">打印机</th>
                <th class="py-4 px-4 font-normal">状态</th>
                <th class="py-4 px-4 font-normal">打印时间</th>
                <th class="py-4 px-4 font-normal">操作</th>
              </tr>
            </thead>
            <tbody>
              <!-- 空数据状态 -->
              <tr>
                <td colspan="7" class="py-20 text-center">
                  <div class="flex flex-col items-center justify-center">
                    <p class="text-gray-500 text-base mb-1">暂无打印历史记录</p>
                    <p class="text-gray-400 text-sm">开始打印文件后，历史记录将显示在这里</p>
                  </div>
                </td>
              </tr>
              <!-- 如果有数据，可以在这里使用 v-for 渲染 <tr> -->
            </tbody>
          </table>
        </div>
      </div>
    </main>

    <!-- 底部版权信息 -->
    <footer class="absolute bottom-4 w-full text-center text-xs text-gray-400 z-10">
      © 2025 打印助手 · 安全、高效的在线打印解决方案
    </footer>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// 目前页面为静态展示，无需复杂的逻辑
// 如果后续需要接入数据，可以在这里定义 ref 和请求方法

// 示例：
// import { ref, onMounted } from 'vue'
// const printRecords = ref([])
// const fetchRecords = async () => { ... }
// onMounted(() => fetchRecords())
</script>

<style scoped>
/* 如果您需要一些特定的非 Tailwind 样式，可以写在这里 */
</style>