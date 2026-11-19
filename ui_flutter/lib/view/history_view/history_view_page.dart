import 'package:get/get.dart';
import 'package:flutter/material.dart';

import './history_view_controller.dart';

class HistoryViewPage extends GetView<HistoryViewController> {
  const HistoryViewPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Obx(() {
        if (controller.history.isEmpty) {
          return const Center(child: Text('暂无打印历史'));
        }
        return ListView.builder(
          itemCount: controller.history.length,
          itemBuilder: (context, index) {
            final item = controller.history[index];
            return ListTile(
              title: Text(item.job_name),
              subtitle: Text(item.printer_name),
              trailing: Text(item.status),
            );
          },
        );
      }),
    );
  }
}
