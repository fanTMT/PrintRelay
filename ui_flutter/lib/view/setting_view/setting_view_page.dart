import 'package:get/get.dart';
import 'package:flutter/material.dart';

import './setting_view_controller.dart';

class SettingViewPage extends GetView<SettingViewController> {
  const SettingViewPage({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(body: Center(child: Text('设置')));
  }
}
