import 'package:get/get.dart';
import 'package:ui_flutter/data/history.dart';

class HistoryViewController extends GetxController {
  final RxList<HistoryItem> history = <HistoryItem>[].obs;

  @override
  void onInit() {
    super.onInit();
    loadHistory();
  }

  void loadHistory() {
    history.assignAll([
      HistoryItem(
        id: 1,
        job_name: 'Item 1',
        printer_name: '打印机 A',
        file_path: '/tmp/item-1.pdf',
        status: 'completed',
        cups_job_id: 2,
        error_reason: '',
        created_at: '2026-11-19 10:00:00',
        updated_at: '2026-11-19 10:01:00',
        finished_at: '2026-11-19 10:01:00',
      ),
      HistoryItem(
        id: 2,
        job_name: 'Item 2',
        printer_name: '打印机 B',
        file_path: '/tmp/item-2.pdf',
        status: 'failed',
        cups_job_id: 3,
        error_reason: '打印机离线',
        created_at: '2026-11-19 09:00:00',
        updated_at: '2026-11-19 09:01:00',
        finished_at: '2026-11-19 09:01:00',
      ),
      HistoryItem(
        id: 3,
        job_name: 'Item 3',
        printer_name: '打印机 A',
        file_path: '/tmp/item-3.pdf',
        status: 'pending',
        cups_job_id: 4,
        error_reason: '',
        created_at: '2026-11-19 08:00:00',
        updated_at: '2026-11-19 08:00:00',
        finished_at: '',
      ),
    ]);
  }
}
