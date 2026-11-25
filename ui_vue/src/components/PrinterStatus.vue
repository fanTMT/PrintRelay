<template>
  <div class="rounded-2xl border border-slate-100 bg-white p-6 text-slate-800 shadow-[0_12px_32px_rgba(15,23,42,0.08)]" v-for="print in printers">
    <div class="mb-5 flex items-center justify-between">
      <div class="flex items-center gap-2.5 font-bold text-slate-900" >
        <span aria-hidden="true" class="h-2.5 w-2.5 rounded-full"
          :class="print.state === 'printing' ? 'bg-emerald-500' : print.state === 'paused' ? 'bg-amber-500' : print.state === 'error' ? 'bg-red-500' : print.state === 'offline' ? 'bg-slate-400' : 'bg-sky-500'"></span>
        {{ print.name }}
      </div>
      <span class="rounded-full px-3 py-1 text-xs font-semibold"
        :class="print.state === 'printing' ? 'bg-emerald-50 text-emerald-700' : print.state === 'paused' ? 'bg-amber-50 text-amber-700' : print.state === 'error' ? 'bg-red-50 text-red-700' : print.state === 'offline' ? 'bg-slate-100 text-slate-600' : 'bg-sky-50 text-sky-700'">
        {{ print.is_printing ? '正在打印' : print.state === 'paused' ? '已暂停' : print.state === 'error' ? '错误' : print.state === 'offline' ? '离线' : '空闲' }}
      </span>
    </div>

    <div class="mb-2 flex items-center justify-between text-sm">
      <span class="text-slate-600">{{ print.state }}</span>
      <!-- <strong class="text-slate-900">{{ print. }}%</strong> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { ref } from 'vue'
import axios from 'axios'

interface PrinterStatus {
  name: string
  state: string
  is_printing: boolean
  is_def:boolean
}

const printers = ref<PrinterStatus[]>([])

const fetchPrinterStatus = () => {
  axios.get('/api/print-status')
    .then(response => {
      const list = response.data
      if (!Array.isArray(list)) {
        console.warn('返回格式不是数组:', list)
        return
      }
      const tmp :any[]= [];

      list.forEach(p => {
        tmp.push({
          name: p.name,
          state: p.state,
          is_printing: p.is_printing
        })  
      })
      printers.value= tmp;
    })
    .catch(error => {
      console.error('获取打印机状态失败:', error)
    })
}

onMounted(() => {
  fetchPrinterStatus()
  setInterval(fetchPrinterStatus, 5000) // 每5秒刷新一次状态
})


</script>
