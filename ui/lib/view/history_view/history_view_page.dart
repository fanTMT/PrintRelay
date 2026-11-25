import 'package:get/get.dart';
import 'package:ui_flutter/data/history.dart';
import 'package:flutter/material.dart';

import './history_view_controller.dart';

class HistoryViewPage extends GetView<HistoryViewController> {
  const HistoryViewPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      if (controller.isLoading.value && controller.history.isEmpty) {
        return const Center(child: CircularProgressIndicator());
      }
      if (controller.errorMessage.value.isNotEmpty &&
          controller.history.isEmpty) {
        return Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(controller.errorMessage.value),
              const SizedBox(height: 12),
              OutlinedButton.icon(
                onPressed: controller.loadHistory,
                icon: const Icon(Icons.refresh),
                label: const Text('重试'),
              ),
            ],
          ),
        );
      }
      if (controller.history.isEmpty) {
        return const Center(child: Text('暂无打印历史记录'));
      }
      final records = controller.filteredHistory;
      final wide = MediaQuery.sizeOf(context).width >= 760;
      return RefreshIndicator(
        onRefresh: controller.loadHistory,
        child: ListView(
          padding: const EdgeInsets.all(24),
          children: [
            Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 1120),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Wrap(
                      spacing: 14,
                      runSpacing: 14,
                      children: [
                        _StatCard(
                          title: '总打印次数',
                          value: '${controller.history.length}',
                          detail: '本月：${controller.currentMonthCount} 次',
                          icon: Icons.receipt_long_outlined,
                          color: const Color(0xff5965e8),
                        ),
                        _StatCard(
                          title: '已完成打印',
                          value: '${controller.completedCount}',
                          detail: controller.history.isEmpty
                              ? '成功率：--'
                              : '成功率：${(controller.completedCount * 100 / controller.history.length).round()}%',
                          icon: Icons.task_alt,
                          color: const Color(0xff16845b),
                        ),
                        const _StatCard(
                          title: '打印文件大小',
                          value: '--',
                          detail: '服务端未提供文件大小',
                          icon: Icons.insert_drive_file_outlined,
                          color: Color(0xff3973ba),
                        ),
                      ],
                    ),
                    const SizedBox(height: 22),
                    Card(
                      color: Colors.white,
                      child: Padding(
                        padding: const EdgeInsets.all(20),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            Wrap(
                              alignment: WrapAlignment.spaceBetween,
                              runSpacing: 12,
                              spacing: 12,
                              crossAxisAlignment: WrapCrossAlignment.center,
                              children: [
                                Text(
                                  '历史记录列表',
                                  style: Theme.of(context).textTheme.titleMedium
                                      ?.copyWith(fontWeight: FontWeight.w700),
                                ),
                                SizedBox(
                                  width: wide ? 280 : 220,
                                  child: TextField(
                                    onChanged: (value) =>
                                        controller.searchQuery.value = value,
                                    decoration: const InputDecoration(
                                      isDense: true,
                                      prefixIcon: Icon(Icons.search),
                                      hintText: '搜索文件名',
                                      border: OutlineInputBorder(),
                                    ),
                                  ),
                                ),
                                Wrap(
                                  spacing: 8,
                                  children: [
                                    IconButton(
                                      tooltip: '刷新记录',
                                      onPressed: controller.loadHistory,
                                      icon: const Icon(Icons.refresh),
                                    ),
                                    OutlinedButton.icon(
                                      onPressed: controller.history.isEmpty
                                          ? null
                                          : () => _confirmClear(context),
                                      icon: const Icon(Icons.delete_outline),
                                      label: const Text('清空列表'),
                                    ),
                                  ],
                                ),
                              ],
                            ),
                            const SizedBox(height: 16),
                            if (records.isEmpty)
                              const Padding(
                                padding: EdgeInsets.all(28),
                                child: Text(
                                  '没有匹配的记录',
                                  textAlign: TextAlign.center,
                                ),
                              )
                            else if (wide)
                              SingleChildScrollView(
                                scrollDirection: Axis.horizontal,
                                child: DataTable(
                                  columns: const [
                                    DataColumn(label: Text('ID')),
                                    DataColumn(label: Text('文件名')),
                                    DataColumn(label: Text('打印机')),
                                    DataColumn(label: Text('状态')),
                                    DataColumn(label: Text('打印时间')),
                                    DataColumn(label: Text('操作')),
                                  ],
                                  rows: records
                                      .map(
                                        (record) => DataRow(
                                          cells: [
                                            DataCell(Text('${record.id}')),
                                            DataCell(Text(record.jobName)),
                                            DataCell(Text(record.printerName)),
                                            DataCell(
                                              _StatusText(
                                                status: record.status,
                                              ),
                                            ),
                                            DataCell(Text(record.createdAt)),
                                            DataCell(
                                              TextButton(
                                                onPressed: () => _showDetails(
                                                  context,
                                                  record,
                                                ),
                                                child: const Text('查看详情'),
                                              ),
                                            ),
                                          ],
                                        ),
                                      )
                                      .toList(),
                                ),
                              )
                            else
                              ...records.map(
                                (record) => Card.outlined(
                                  child: ListTile(
                                    title: Text(record.jobName),
                                    subtitle: Text(
                                      '${record.printerName} · ${record.createdAt}',
                                    ),
                                    trailing: _StatusText(
                                      status: record.status,
                                    ),
                                    onTap: () => _showDetails(context, record),
                                  ),
                                ),
                              ),
                          ],
                        ),
                      ),
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

  Future<void> _confirmClear(BuildContext context) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('清空当前列表？'),
        content: const Text('只会清除本次页面中显示的数据，不会删除服务器记录。'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('取消'),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text('清空'),
          ),
        ],
      ),
    );
    if (confirmed == true) controller.clearHistory();
  }

  void _showDetails(BuildContext context, HistoryItem record) {
    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(record.jobName),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('打印机：${record.printerName}'),
            Text('状态：${record.status}'),
            Text('路径：${record.filePath}'),
            Text('创建时间：${record.createdAt}'),
            if (record.errorReason?.isNotEmpty == true)
              Text('错误信息：${record.errorReason}'),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('关闭'),
          ),
        ],
      ),
    );
  }
}

class _StatCard extends StatelessWidget {
  const _StatCard({
    required this.title,
    required this.value,
    required this.detail,
    required this.icon,
    required this.color,
  });

  final String title;
  final String value;
  final String detail;
  final IconData icon;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 260,
      child: Card(
        color: Colors.white,
        child: Padding(
          padding: const EdgeInsets.all(18),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Expanded(child: Text(title)),
                  Icon(icon, color: color),
                ],
              ),
              const SizedBox(height: 14),
              Text(value, style: Theme.of(context).textTheme.headlineSmall),
              const SizedBox(height: 4),
              Text(
                detail,
                style: Theme.of(context).textTheme.bodySmall
                    ?.copyWith(color: const Color(0xff788196)),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _StatusText extends StatelessWidget {
  const _StatusText({required this.status});

  final String status;

  @override
  Widget build(BuildContext context) {
    final isDone = status.toLowerCase() == 'completed';
    return Text(
      status,
      style: TextStyle(
        color: isDone ? const Color(0xff16845b) : const Color(0xffa66b08),
        fontWeight: FontWeight.w600,
      ),
    );
  }
}
