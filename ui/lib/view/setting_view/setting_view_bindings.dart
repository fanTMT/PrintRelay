import 'package:get/get.dart';

import './setting_view_controller.dart';

class SettingViewBindings implements Bindings {
  @override
  void dependencies() {
    Get.put(SettingViewController());
  }
}
