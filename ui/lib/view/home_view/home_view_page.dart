import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:pdfx/pdfx.dart';
import 'package:ui_flutter/view/about_view/about_view_page.dart';
import 'package:ui_flutter/view/history_view/history_view_page.dart';
import 'package:ui_flutter/view/home_view/login_view_page.dart';
import 'package:ui_flutter/view/printer_view/printer_view_page.dart';

import './home_view_controller.dart';

class HomeViewPage extends GetView<HomeViewController> {
  const HomeViewPage({super.key});

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final isWide = constraints.maxWidth >= 760;
        return Obx(() {
          final loginVisible = controller.showLogin.value;
          final selectedIndex = controller.selectedIndex.value;
          return Scaffold(
            appBar: AppBar(
              title: Text(loginVisible ? '登录' : '共享打印服务'),
              leading: loginVisible
                  ? IconButton(
                      tooltip: '返回',
                      onPressed: () => controller.showLogin.value = false,
                      icon: const Icon(Icons.arrow_back),
                    )
                  : null,
              actions: [
                if (!loginVisible && isWide)
                  for (
                    var index = 0;
                    index < HomeViewController.tabs.length;
                    index++
                  )
                    TextButton(
                      onPressed: () => controller.selectedIndex.value = index,
                      style: TextButton.styleFrom(
                        foregroundColor: selectedIndex == index
                            ? Theme.of(context).colorScheme.primary
                            : const Color(0xff697386),
                      ),
                      child: Text(HomeViewController.tabs[index]),
                    ),
                if (!loginVisible)
                  Padding(
                    padding: const EdgeInsets.only(left: 12, right: 16),
                    child: OutlinedButton.icon(
                      onPressed: () => controller.showLogin.value = true,
                      icon: const Icon(Icons.person_outline, size: 18),
                      label: const Text('未登录 · 登录'),
                    ),
                  ),
              ],
            ),
            body: loginVisible
                ? const LoginViewPage()
                : switch (selectedIndex) {
                    0 => _HomeContent(controller: controller),
                    1 => const HistoryViewPage(),
                    2 => const PrinterViewPage(),
                    3 => const AboutViewPage(),
                    _ => _HomeContent(controller: controller),
                  },
            bottomNavigationBar: !loginVisible && !isWide
                ? NavigationBar(
                    selectedIndex: selectedIndex,
                    onDestinationSelected: (index) =>
                        controller.selectedIndex.value = index,
                    destinations: const [
                      NavigationDestination(
                        icon: Icon(Icons.home_outlined),
                        selectedIcon: Icon(Icons.home),
                        label: '首页',
                      ),
                      NavigationDestination(
                        icon: Icon(Icons.history_outlined),
                        selectedIcon: Icon(Icons.history),
                        label: '历史记录',
                      ),
                      NavigationDestination(
                        icon: Icon(Icons.print_outlined),
                        selectedIcon: Icon(Icons.print),
                        label: '打印机',
                      ),
                      NavigationDestination(
                        icon: Icon(Icons.info_outline),
                        selectedIcon: Icon(Icons.info),
                        label: '关于我们',
                      ),
                    ],
                  )
                : null,
          );
        });
      },
    );
  }
}

class _HomeContent extends StatelessWidget {
  const _HomeContent({required this.controller});

  final HomeViewController controller;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 920),
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Row(
                children: [
                  Text(
                    '文件预览',
                    style: Theme.of(context).textTheme.titleLarge
                        ?.copyWith(fontWeight: FontWeight.w700),
                  ),
                  const Spacer(),
                  Obx(() {
                    final file = controller.selectedFile.value;
                    if (file == null) {
                      return FilledButton.icon(
                        onPressed: controller.pickFile,
                        icon: const Icon(Icons.folder_open),
                        label: const Text('选择本地文件'),
                      );
                    }
                    return Wrap(
                      spacing: 8,
                      children: [
                        OutlinedButton.icon(
                          onPressed: controller.clearFile,
                          icon: const Icon(Icons.close),
                          label: const Text('取消'),
                        ),
                        FilledButton.icon(
                          onPressed: controller.showUploadUnavailable,
                          icon: const Icon(Icons.cloud_upload_outlined),
                          label: const Text('上传到服务器'),
                        ),
                      ],
                    );
                  }),
                ],
              ),
              const SizedBox(height: 8),
              Obx(() {
                final fileName = controller.selectedFile.value?.name;
                return Text(
                  fileName == null
                      ? '支持 PDF、图片、Word、TXT 等文件'
                      : '当前文件：$fileName',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: Theme.of(context).textTheme.bodySmall
                      ?.copyWith(color: const Color(0xff697386)),
                );
              }),
              const SizedBox(height: 18),
              Expanded(
                child: Obx(() {
                  final file = controller.selectedFile.value;
                  final bytes = controller.fileBytes.value;
                  if (file == null || bytes == null) {
                    return InkWell(
                      onTap: controller.pickFile,
                      borderRadius: BorderRadius.circular(12),
                      child: Container(
                        constraints: const BoxConstraints(minHeight: 300),
                        decoration: BoxDecoration(
                          color: const Color(0xfff5f7fa),
                          border: Border.all(
                            color: const Color(0xffcbd3df),
                            style: BorderStyle.solid,
                          ),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: const Center(
                          child: Column(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Icon(Icons.upload_file_outlined, size: 42),
                              SizedBox(height: 12),
                              Text('选择文件以预览'),
                            ],
                          ),
                        ),
                      ),
                    );
                  }
                  final extension = file.extension?.toLowerCase();
                  if (extension == 'pdf') {
                    final pdfController = controller.pdfController.value;
                    if (pdfController == null) {
                      return const Center(child: CircularProgressIndicator());
                    }
                    return ClipRRect(
                      borderRadius: BorderRadius.circular(10),
                      child: PdfViewPinch(controller: pdfController),
                    );
                  }
                  if (const [
                    'png',
                    'jpg',
                    'jpeg',
                    'gif',
                    'webp',
                  ].contains(extension)) {
                    return Container(
                      alignment: Alignment.center,
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        color: const Color(0xffe9e9eb),
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Image.memory(bytes, fit: BoxFit.contain),
                    );
                  }
                  return Center(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        const Icon(Icons.description_outlined, size: 48),
                        const SizedBox(height: 12),
                        Text('当前文件不支持在线预览：${file.name}'),
                        const SizedBox(height: 6),
                        const Text('可使用“上传到服务器”继续处理。'),
                      ],
                    ),
                  );
                }),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
