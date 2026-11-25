<script setup lang="ts">
import { ref, onUnmounted, computed } from 'vue'
import VuePdfEmbed from 'vue-pdf-embed'

// ✅ 引入 PDF 插件自带的样式
import 'vue-pdf-embed/dist/styles/textLayer.css'
import 'vue-pdf-embed/dist/styles/annotationLayer.css'

// --- 状态数据 ---
const pdfSource = ref<string>('')
const fileName = ref<string>('')
const currentFile = ref<File | null>(null)
const isUploading = ref(false)
const pdfError = ref('')

// --- 计算属性：精准识别文件类型，用于控制模板渲染 ---
const fileType = computed(() => {
  if (!fileName.value) return ''
  const ext = fileName.value.split('.').pop()?.toLowerCase() || ''
  
  if (ext === 'pdf') return 'pdf'
  if (['png', 'jpg', 'jpeg', 'gif', 'webp'].includes(ext)) return 'image'
  return 'other'
})

// --- 核心方法 ---
// 处理文件选择
const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  // 允许的后缀列表
  const allowedExtensions = ['pdf', 'doc', 'docx', 'txt', 'md', 'png', 'jpg', 'jpeg']
  const fileExtension = file.name.split('.').pop()?.toLowerCase()

  if (!fileExtension || !allowedExtensions.includes(fileExtension)) {
    alert('请选择有效的文件（支持 PDF、图片、Word、TXT 等）！')
    return
  }

  // 清理旧的 URL 防止内存泄漏
  if (pdfSource.value && pdfSource.value.startsWith('blob:')) {
    URL.revokeObjectURL(pdfSource.value)
  }

  fileName.value = file.name
  currentFile.value = file
  pdfError.value = ''
  // 生成 Blob URL 进行预览
  pdfSource.value = URL.createObjectURL(file)
  
  // 清空 input 允许重复选同一文件
  target.value = '' 
}

// 取消/重置
const handleCancel = () => {
  if (pdfSource.value && pdfSource.value.startsWith('blob:')) {
    URL.revokeObjectURL(pdfSource.value)
  }
  pdfSource.value = ''
  fileName.value = ''
  currentFile.value = null
  pdfError.value = ''
}

// PDF.js can fail to load or render a document; surface the failure instead of leaving a blank preview.
const handlePdfError = (error: unknown) => {
  console.error('PDF preview failed:', error)
  pdfError.value = 'PDF 预览失败，请确认文件未损坏后重试。'
}

// 上传到服务器
const handleUpload = async () => {
  if (!currentFile.value) return
  isUploading.value = true
  
  try {
    // 💡 真实上传时替换这段代码，使用 FormData
    // const formData = new FormData()
    // formData.append('file', currentFile.value)
    // await fetch('/api/upload', { method: 'POST', body: formData })
    
    await new Promise((resolve) => setTimeout(resolve, 1500)) // 模拟请求耗时
    alert(`文件 "${fileName.value}" 上传成功！`)
  } catch (error) {
    alert('上传失败，请重试')
  } finally {
    isUploading.value = false
  }
}

// 触发隐藏的 input
const triggerUpload = () => {
  document.getElementById('pdf-upload')?.click()
}

// 组件卸载时清理内存
onUnmounted(() => {
  if (pdfSource.value && pdfSource.value.startsWith('blob:')) {
    URL.revokeObjectURL(pdfSource.value)
  }
})
</script>

<template>
  <div class="mx-auto w-full max-w-[800px] p-5">
    <div class="mb-5 flex items-center justify-between gap-3 max-sm:flex-wrap">
      <div class="flex items-center gap-2.5">
        <input
          type="file"
          accept=".pdf,.doc,.docx,.txt,.md,.png,.jpg,.jpeg"
          id="pdf-upload"
          class="hidden"
          @change="handleFileChange"
        />
        <label v-if="!pdfSource" for="pdf-upload" class="cursor-pointer rounded border border-transparent bg-[#409eff] px-4 py-2 text-sm text-white transition-colors hover:bg-[#66b1ff]">📁 选择本地文件</label>
        <span v-if="fileName" class="max-w-[250px] truncate text-sm text-[#606266]" :title="fileName">当前文件: {{ fileName }}</span>
      </div>

      <div class="flex items-center gap-2.5" v-if="pdfSource">
        <button class="rounded border border-[#f56c6c] bg-white px-4 py-2 text-sm text-[#f56c6c] transition-colors hover:bg-[#fef0f0] disabled:cursor-not-allowed disabled:opacity-60" @click="handleCancel" :disabled="isUploading">取消</button>
        <button class="rounded border border-transparent bg-[#67c23a] px-4 py-2 text-sm text-white transition-colors hover:bg-[#85ce61] disabled:cursor-not-allowed disabled:opacity-60" @click="handleUpload" :disabled="isUploading">
          {{ isUploading ? '上传中...' : '上传到服务器' }}
        </button>
      </div>
    </div>

    <div class="min-h-[400px] overflow-hidden rounded-md border border-[#ebeef5] bg-[#f5f7fa]">
      <div v-if="pdfSource && fileType === 'pdf'">
        <VuePdfEmbed
          :key="pdfSource"
          :source="pdfSource"
          @loading-failed="handlePdfError"
          @rendering-failed="handlePdfError"
        />
        <p v-if="pdfError" class="flex min-h-[400px] flex-col items-center justify-center px-5 py-12 text-center text-[#606266]" role="alert">{{ pdfError }}</p>
      </div>

      <div v-else-if="pdfSource && fileType === 'image'" class="flex min-h-[400px] items-center justify-center bg-[#e9e9eb] p-5">
        <img class="max-h-[600px] max-w-full rounded object-contain shadow-[0_4px_12px_rgba(0,0,0,0.1)]" :src="pdfSource" :alt="fileName" />
      </div>

      <div v-else-if="pdfSource" class="flex min-h-[400px] flex-col items-center justify-center bg-white px-5 py-12 text-center text-[#606266]">
        <p class="my-2 text-base">📄 当前文件不支持在线预览：{{ fileName }}</p>
        <p class="my-2 text-sm text-[#909399]">请点击右上方“上传到服务器”进行后续操作。</p>
      </div>

      <div v-else class="grid h-[400px] cursor-pointer place-items-center rounded-md border-2 border-dashed border-[#dcdfe6] text-center text-[#909399] transition-colors hover:border-[#409eff] hover:bg-[#f0f7ff] hover:text-[#409eff]" @click="triggerUpload">
        请点击此处或上方按钮选择文件进行预览
      </div>
    </div>
  </div>
</template>