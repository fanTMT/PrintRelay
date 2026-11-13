<template>
  <div class="printer-status" :class="`status-${status}`">
    <div class="printer-status__header">
      <div class="printer-status__title">
        <span class="printer-status__dot" aria-hidden="true"></span>
        {{ printerName }}
      </div>
      <span class="printer-status__badge" :class="`badge-${status}`">
        {{ statusText }}
      </span>
    </div>

    <div class="printer-status__job">
      <span>{{ currentJob }}</span>
      <strong>{{ percent }}%</strong>
    </div>

    <div class="printer-status__progress" aria-label="打印进度">
      <div class="printer-status__progress-bar" :style="{ width: `${percent}%` }"></div>
    </div>

    <div class="printer-status__meta">
      <span>{{ elapsed }}</span>
      <span>{{ total }}</span>
    </div>

    <div class="printer-status__stats">
      <div class="printer-status__stat">
        <label>喷头</label>
        <span>{{ nozzleTemp }}°C</span>
      </div>
      <div class="printer-status__stat">
        <label>热床</label>
        <span>{{ bedTemp }}°C</span>
      </div>
      <div class="printer-status__stat">
        <label>耗材</label>
        <span>{{ material }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  printerName: {
    type: String,
    default: 'Printer'
  },
  status: {
    type: String,
    default: 'idle',
    validator: (value) => ['idle', 'printing', 'paused', 'error', 'offline'].includes(value)
  },
  currentJob: {
    type: String,
    default: 'No active job'
  },
  progress: {
    type: Number,
    default: 0
  },
  elapsed: {
    type: String,
    default: '00:00'
  },
  total: {
    type: String,
    default: '00:00'
  },
  nozzleTemp: {
    type: Number,
    default: 0
  },
  bedTemp: {
    type: Number,
    default: 0
  },
  material: {
    type: String,
    default: 'PLA'
  }
})

const percent = computed(() => {
  const value = Number(props.progress) || 0
  return Math.max(0, Math.min(100, value))
})

const statusText = computed(() => {
  const map = {
    idle: '空闲',
    printing: '打印中',
    paused: '暂停',
    error: '错误',
    offline: '离线'
  }

  return map[props.status] || '未知'
})
</script>

<style scoped>
.printer-status {
  width: 100%;
  max-width: 360px;
  padding: 16px;
  border-radius: 14px;
  background: linear-gradient(180deg, #1f2937 0%, #111827 100%);
  color: #f9fafb;
  box-shadow: 0 12px 28px rgba(15, 23, 42, 0.18);
  border: 1px solid rgba(148, 163, 184, 0.2);
  box-sizing: border-box;
}

.printer-status__header,
.printer-status__job,
.printer-status__meta,
.printer-status__stats {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.printer-status__header {
  margin-bottom: 12px;
}

.printer-status__title {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
}

.printer-status__dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #22c55e;
  box-shadow: 0 0 10px rgba(34, 197, 94, 0.8);
}

.status-paused .printer-status__dot {
  background: #fbbf24;
  box-shadow: 0 0 10px rgba(251, 191, 36, 0.8);
}

.status-error .printer-status__dot,
.status-offline .printer-status__dot {
  background: #ef4444;
  box-shadow: 0 0 10px rgba(239, 68, 68, 0.8);
}

.status-idle .printer-status__dot {
  background: #60a5fa;
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.8);
}

.printer-status__badge {
  padding: 4px 8px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 600;
}

.badge-printing {
  background: rgba(34, 197, 94, 0.15);
  color: #86efac;
}

.badge-idle {
  background: rgba(96, 165, 250, 0.15);
  color: #93c5fd;
}

.badge-paused {
  background: rgba(251, 191, 36, 0.15);
  color: #fcd34d;
}

.badge-error,
.badge-offline {
  background: rgba(239, 68, 68, 0.15);
  color: #fca5a5;
}

.printer-status__job {
  margin-bottom: 12px;
  font-size: 13px;
  color: #d1d5db;
}

.printer-status__job strong {
  font-size: 14px;
  color: #f8fafc;
}

.printer-status__progress {
  height: 10px;
  border-radius: 999px;
  background: rgba(148, 163, 184, 0.18);
  overflow: hidden;
  margin-bottom: 10px;
}

.printer-status__progress-bar {
  height: 100%;
  border-radius: inherit;
  background: linear-gradient(90deg, #22c55e 0%, #38bdf8 100%);
  transition: width 0.3s ease;
}

.printer-status__meta {
  font-size: 12px;
  color: #cbd5e1;
  margin-bottom: 14px;
}

.printer-status__stats {
  gap: 8px;
}

.printer-status__stat {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px 8px;
  border-radius: 10px;
  background: rgba(15, 23, 42, 0.4);
  text-align: center;
}

.printer-status__stat label {
  font-size: 11px;
  color: #94a3b8;
}

.printer-status__stat span {
  font-size: 13px;
  font-weight: 600;
  color: #f8fafc;
}
</style>
