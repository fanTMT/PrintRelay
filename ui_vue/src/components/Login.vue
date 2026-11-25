<template>
	<main class="grid min-h-screen place-items-center bg-[#f4f6fb] px-5 py-8 font-sans text-[#20283a]">
		<section class="w-full max-w-[420px] rounded-2xl border border-[#e8ebf2] bg-white p-[30px] shadow-[0_16px_48px_rgb(34_51_84_/_9%)] sm:p-10" aria-labelledby="login-title">
			<div class="mb-6 grid size-11 place-items-center rounded-xl bg-[#4263eb] text-[22px] font-bold text-white" aria-hidden="true">P</div>
			<h1 id="login-title" class="m-0 text-[26px] font-bold">欢迎回来</h1>
			<p class="mb-[30px] mt-[9px] text-sm text-[#7a8498]">登录你的账号以继续</p>

			<form @submit.prevent="handleLogin">
				<label for="email" class="mb-2 block text-[13px] font-semibold text-[#39445a]">邮箱或用户名</label>
				<input
					id="email"
					v-model.trim="email"
					type="text"
					autocomplete="username"
					placeholder="请输入邮箱或用户名"
					class="mb-5 h-[46px] w-full rounded-lg border border-[#dce1eb] px-[13px] text-sm text-[#20283a] outline-none transition focus:border-[#4263eb] focus:ring-4 focus:ring-[#4263eb]/10 placeholder:text-[#a6adbb]"
					required
				/>

				<div class="flex items-center justify-between">
					<label for="password" class="mb-2 block text-[13px] font-semibold text-[#39445a]">密码</label>
					<a class="text-[13px] text-[#4263eb] no-underline hover:underline" href="#forgot-password" @click.prevent="$emit('forgot-password')">忘记密码？</a>
				</div>
				<input
					id="password"
					v-model="password"
					type="password"
					autocomplete="current-password"
					placeholder="请输入密码"
					class="mb-5 h-[46px] w-full rounded-lg border border-[#dce1eb] px-[13px] text-sm text-[#20283a] outline-none transition focus:border-[#4263eb] focus:ring-4 focus:ring-[#4263eb]/10 placeholder:text-[#a6adbb]"
					required
				/>

				<p v-if="errorMessage" class="-mt-2 mb-[14px] text-[13px] text-[#d14343]" role="alert">{{ errorMessage }}</p>
				<button class="h-[46px] w-full rounded-lg border-0 bg-[#4263eb] text-[15px] font-semibold text-white transition hover:bg-[#3451cf] disabled:cursor-not-allowed disabled:opacity-70" type="submit" :disabled="isSubmitting">
					{{ isSubmitting ? '登录中…' : '登 录' }}
				</button>
			</form>

			<p class="mb-0 mt-6 text-center text-[13px] text-[#7a8498]">还没有账号？ <a class="text-[13px] text-[#4263eb] no-underline hover:underline" href="#register" @click.prevent="$emit('register')">立即注册</a></p>
		</section>
	</main>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const emit = defineEmits<{
	login: [credentials: { username: string; password: string }]
	'forgot-password': []
	register: []
}>()

const email = ref('')
const password = ref('')
const isSubmitting = ref(false)
const errorMessage = ref('')

function handleLogin() {
	if (isSubmitting.value) return
	errorMessage.value = ''
	emit('login', {
		username: email.value,
		password: password.value,
	})
}
</script>

