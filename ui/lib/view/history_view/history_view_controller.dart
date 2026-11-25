import 'package:get/get.dart';
import 'package:ui_flutter/data/api_service.dart';
import 'package:ui_flutter/data/history.dart';

class HistoryViewController extends GetxController {
  final ApiService _api = ApiService();
  final RxList<HistoryItem> history = <HistoryItem>[].obs;
  final isLoading = false.obs;
  final errorMessage = ''.obs;
  final searchQuery = ''.obs;

  List<HistoryItem> get filteredHistory {
    final query = searchQuery.value.trim().toLowerCase();
    if (query.isEmpty) return history.toList();
    return history
        .where((item) => item.jobName.toLowerCase().contains(query))
        .toList();
  }

  int get completedCount =>
      history.where((item) => item.status.toLowerCase() == 'completed').length;

  int get currentMonthCount {
    final now = DateTime.now();
    return history.where((item) {
      final date = DateTime.tryParse(item.createdAt.replaceFirst(' ', 'T'));
      return date != null && date.year == now.year && date.month == now.month;
    }).length;
  }

  void clearHistory() => history.clear();

  @override
  void onInit() {
    super.onInit();
    loadHistory();
  }

  Future<void> loadHistory() async {
    if (isLoading.value) return;
    isLoading.value = true;
    errorMessage.value = '';
    try {
      history.assignAll(await _api.fetchHistory());
    } catch (_) {
      errorMessage.value = '无法连接打印服务，请检查服务地址后重试。';
    } finally {
      isLoading.value = false;
    }
  }
}
