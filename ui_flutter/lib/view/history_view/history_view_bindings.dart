import 'package:get/get.dart';

import './history_view_controller.dart';

class HistoryViewBindings implements Bindings {
  @override
  void dependencies() {
    Get.put(HistoryViewController());
  }
}
