<template>
	<div class="page">
		<header class="topbar">
			<div class="brand">Vue UI</div>
			<nav class="switch-bar" aria-label="页面导航">
				<button
					v-for="item in tabs"
					:key="item"
					class="tab"
					:class="{ active: activeTab === item }"
					:aria-current="activeTab === item ? 'page' : undefined"
					@click="activeTab = item"
				>
					{{ item }}
				</button>
			</nav>
			<button class="profile" aria-label="个人中心">A</button>
		</header>

		<main class="content">
			<PdfPreview v-if="activeTab === '首页'" />
      <History v-else-if="activeTab === '历史记录'" />
      <PrinterStatus v-else-if="activeTab === '打印机状态管理'" />
      <AboutUs v-else-if="activeTab === '关于我们'" />
			<template v-else>
			<p class="eyebrow">欢迎使用</p>
			<h1>{{ activeTab }}</h1>
			<p class="description">这是一个简洁的头部切换栏示例，点击上方导航切换页面内容。</p>
			<div class="placeholder-card">
				<span class="card-icon">✦</span>
				<div>
					<strong>{{ activeTab }}页面</strong>
					<p>当前已切换至「{{ activeTab }}」</p>
				</div>
			</div>
			</template>
		</main>
	</div>
</template>

<script setup>
import { ref } from 'vue'
import PdfPreview from './components/PdfPreview.vue'
import History from './components/History.vue';
import PrinterStatus from './components/PrinterStatus.vue';
import AboutUs from './components/AboutUs.vue';

const tabs = ['首页', '历史记录', '打印机状态管理', '关于我们']
const activeTab = ref(tabs[0])
</script>

<style>
* {
	box-sizing: border-box;
}

body {
	margin: 0;
	font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
	color: #172033;
	background: #f7f8fc;
}

.page {
	min-height: 100vh;
}

.topbar {
	height: 72px;
	padding: 0 7%;
	display: flex;
	align-items: center;
	justify-content: space-between;
	gap: 32px;
	background: #fff;
	border-bottom: 1px solid #edf0f5;
}

.brand {
	color: #5965e8;
	font-size: 20px;
	font-weight: 750;
	letter-spacing: -0.5px;
}

.switch-bar {
	display: flex;
	align-items: center;
	gap: 8px;
	height: 100%;
}

.tab {
	position: relative;
	height: 100%;
	padding: 0 18px;
	border: 0;
	background: transparent;
	color: #697386;
	font: inherit;
	font-size: 14px;
	cursor: pointer;
	transition: color 0.2s ease;
}

.tab:hover,
.tab.active {
	color: #5965e8;
}

.tab.active {
	font-weight: 650;
}

.tab.active::after {
	position: absolute;
	right: 18px;
	bottom: 0;
	left: 18px;
	height: 3px;
	border-radius: 3px 3px 0 0;
	background: #5965e8;
	content: "";
}

.profile {
	width: 36px;
	height: 36px;
	border: 0;
	border-radius: 50%;
	background: #eef0ff;
	color: #5965e8;
	font-weight: 700;
	cursor: pointer;
}

.content {
	width: min(880px, 86%);
	margin: 96px auto;
}

.eyebrow {
	margin: 0 0 12px;
	color: #5965e8;
	font-size: 13px;
	font-weight: 650;
	letter-spacing: 1px;
}

h1 {
	margin: 0;
	font-size: clamp(36px, 6vw, 52px);
	letter-spacing: -2px;
}

.description {
	margin: 18px 0 40px;
	color: #788196;
	line-height: 1.8;
}

.placeholder-card {
	display: flex;
	align-items: center;
	gap: 18px;
	padding: 26px;
	border: 1px solid #edf0f5;
	border-radius: 14px;
	background: #fff;
	box-shadow: 0 10px 30px rgb(34 45 80 / 4%);
}

.card-icon {
	display: grid;
	width: 46px;
	height: 46px;
	place-items: center;
	border-radius: 12px;
	background: #f0f1ff;
	color: #5965e8;
	font-size: 22px;
}

.placeholder-card strong {
	font-size: 15px;
}

.placeholder-card p {
	margin: 7px 0 0;
	color: #8991a2;
	font-size: 13px;
}

@media (max-width: 600px) {
	.topbar {
		height: auto;
		min-height: 68px;
		padding: 12px 5%;
		flex-wrap: wrap;
		gap: 8px;
	}

	.switch-bar {
		order: 3;
		width: 100%;
		height: 48px;
		justify-content: space-between;
	}

	.tab {
		padding: 0 10px;
	}

	.tab.active::after {
		right: 10px;
		left: 10px;
	}

	.content {
		margin: 64px auto;
	}
}
</style>
