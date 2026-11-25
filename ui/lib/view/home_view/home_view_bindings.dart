import 'package:get/get.dart';

import './home_view_controller.dart';
import '../history_view/history_view_controller.dart';
import '../printer_view/printer_view_controller.dart';
import '../setting_view/setting_view_controller.dart';

class HomeViewBindings implements Bindings {
  @override
  void dependencies() {
    Get.put(HomeViewController());
    Get.lazyPut<HistoryViewController>(() => HistoryViewController());
    Get.lazyPut<PrinterViewController>(() => PrinterViewController());
    Get.lazyPut<SettingViewController>(() => SettingViewController());
  }
}
