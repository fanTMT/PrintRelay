<template>
	<main class="about-us">
		<header class="hero">
			<p class="eyebrow">关于我们</p>
			<h1>用心创造更好的体验</h1>
			<p class="intro">我们专注于打造简单、实用且可靠的产品，帮助每一位用户更轻松地实现自己的目标。</p>
		</header>

		<section class="details" aria-label="关于我们的介绍">
			<article>
				<h2>我们的使命</h2>
				<p>以用户需求为出发点，持续探索更好的解决方案。我们相信，优秀的产品源于倾听、协作与不断改进。</p>
			</article>
			<article>
				<h2>我们的价值观</h2>
				<ul>
					<li><strong>以人为本：</strong>关注真实需求，重视每一次反馈。</li>
					<li><strong>持续改进：</strong>保持好奇，不断学习与迭代。</li>
					<li><strong>真诚协作：</strong>开放沟通，共同创造长期价值。</li>
				</ul>
			</article>
		</section>
	</main>
</template>

<style scoped>
.about-us {
	box-sizing: border-box;
	max-width: 960px;
	margin: 0 auto;
	padding: 64px 24px;
	color: #243044;
}

.hero {
	padding: 48px;
	border-radius: 16px;
	background: #f3f6fb;
	text-align: center;
}

.eyebrow {
	margin: 0 0 12px;
	color: #4778d0;
	font-size: 14px;
	font-weight: 600;
	letter-spacing: 0.08em;
}

h1,
h2 {
	margin-top: 0;
}

h1 {
	margin-bottom: 16px;
	font-size: clamp(28px, 5vw, 40px);
}

.intro {
	max-width: 620px;
	margin: 0 auto;
	color: #647084;
	line-height: 1.8;
}

.details {
	display: grid;
	grid-template-columns: repeat(2, minmax(0, 1fr));
	gap: 40px;
	padding: 48px 8px 0;
}

.details h2 {
	margin-bottom: 14px;
	font-size: 22px;
}

.details p,
.details li {
	color: #647084;
	line-height: 1.8;
}

.details ul {
	margin: 0;
	padding-left: 20px;
}

@media (max-width: 600px) {
	.about-us {
		padding: 32px 16px;
	}

	.hero {
		padding: 36px 22px;
	}

	.details {
		grid-template-columns: 1fr;
		gap: 28px;
		padding-top: 36px;
	}
}
</style>
