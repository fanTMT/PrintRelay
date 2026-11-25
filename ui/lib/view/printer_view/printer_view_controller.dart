import 'dart:async';

import 'package:get/get.dart';
import 'package:ui_flutter/data/api_service.dart';
import 'package:ui_flutter/data/printer.dart';

class PrinterViewController extends GetxController {
  final ApiService _api = ApiService();
  final printers = <PrinterInfo>[].obs;
  final isLoading = false.obs;
  final errorMessage = ''.obs;
  Timer? _refreshTimer;

  @override
  void onInit() {
    super.onInit();
    refreshStatus();
    _refreshTimer = Timer.periodic(
      const Duration(seconds: 5),
      (_) => refreshStatus(),
    );
  }

  Future<void> refreshStatus() async {
    if (isLoading.value) return;
    isLoading.value = true;
    errorMessage.value = '';
    try {
      printers.assignAll(await _api.fetchPrinters());
    } catch (_) {
      errorMessage.value = '无法连接打印服务，请检查服务地址后重试。';
    } finally {
      isLoading.value = false;
    }
  }

  @override
  void onClose() {
    _refreshTimer?.cancel();
    super.onClose();
  }
}
