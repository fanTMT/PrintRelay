import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:ui_flutter/data/printer.dart';

import 'printer_view_controller.dart';

class PrinterViewPage extends GetView<PrinterViewController> {
  const PrinterViewPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      if (controller.isLoading.value && controller.printers.isEmpty) {
        return const Center(child: CircularProgressIndicator());
      }
      if (controller.errorMessage.value.isNotEmpty &&
          controller.printers.isEmpty) {
        return Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(controller.errorMessage.value),
              const SizedBox(height: 12),
              OutlinedButton.icon(
                onPressed: controller.refreshStatus,
                icon: const Icon(Icons.refresh),
                label: const Text('重试'),
              ),
            ],
          ),
        );
      }
      if (controller.printers.isEmpty) {
        return const Center(child: Text('未发现可用打印机'));
      }
      return RefreshIndicator(
        onRefresh: controller.refreshStatus,
        child: ListView(
          padding: const EdgeInsets.all(24),
          children: [
            Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 920),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Row(
                      children: [
                        Expanded(
                          child: Text(
                            '打印机状态管理',
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                        ),
                        IconButton(
                          tooltip: '刷新状态',
                          onPressed: controller.refreshStatus,
                          icon: const Icon(Icons.refresh),
                        ),
                      ],
                    ),
                    const SizedBox(height: 12),
                    ...controller.printers.map(
                      (printer) => _PrinterCard(printer: printer),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      );
    });
  }
}

class _PrinterCard extends StatelessWidget {
  const _PrinterCard({required this.printer});

  final PrinterInfo printer;

  @override
  Widget build(BuildContext context) {
    final color = switch (printer.statusLabel) {
      '正在打印' => const Color(0xff16845b),
      '已暂停' => const Color(0xffa66b08),
      '错误' => const Color(0xffc33e4d),
      '离线' => const Color(0xff737d8c),
      _ => const Color(0xff3973ba),
    };
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      color: Colors.white,
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(Icons.print_outlined, color: color),
                const SizedBox(width: 10),
                Expanded(
                  child: Text(
                    printer.name,
                    style: Theme.of(context).textTheme.titleMedium
                        ?.copyWith(fontWeight: FontWeight.w700),
                  ),
                ),
                DecoratedBox(
                  decoration: BoxDecoration(
                    color: color.withValues(alpha: 0.1),
                    borderRadius: BorderRadius.circular(20),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 12,
                      vertical: 6,
                    ),
                    child: Text(
                      printer.statusLabel,
                      style: TextStyle(
                        color: color,
                        fontSize: 12,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              printer.isDefault ? '${printer.state} · 默认打印机' : printer.state,
              style: Theme.of(context).textTheme.bodySmall
                  ?.copyWith(color: const Color(0xff647084)),
            ),
          ],
        ),
      ),
    );
  }
}
