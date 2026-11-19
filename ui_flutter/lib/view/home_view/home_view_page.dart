import 'package:get/get.dart';
import 'package:flutter/material.dart';
import 'package:ui_flutter/view/history_view/history_view_page.dart';
import 'package:ui_flutter/view/setting_view/setting_view_page.dart';

import './home_view_controller.dart';

class HomeViewPage extends StatefulWidget {
  const HomeViewPage({super.key});

  @override
  State<HomeViewPage> createState() => _HomeViewPageState();
}

class _HomeViewPageState extends State<HomeViewPage> {
  int _selectedIndex = 0;

  static const _titles = ['打印中继', '打印历史', '设置'];

  @override
  Widget build(BuildContext context) {
    final controller = Get.find<HomeViewController>();

    return Scaffold(
      appBar: AppBar(title: Text(_titles[_selectedIndex]), centerTitle: false),
      body: IndexedStack(
        index: _selectedIndex,
        children: [
          Center(
            child: ElevatedButton.icon(
              onPressed: controller.logout,
              icon: const Icon(Icons.logout),
              label: const Text('退出登录'),
            ),
          ),
          const HistoryViewPage(),
          const SettingViewPage(),
        ],
      ),
      bottomNavigationBar: NavigationBar(
        selectedIndex: _selectedIndex,
        labelBehavior: NavigationDestinationLabelBehavior.onlyShowSelected,
        onDestinationSelected: (index) {
          setState(() => _selectedIndex = index);
        },
        destinations: const [
          NavigationDestination(
            icon: Icon(Icons.home_outlined),
            selectedIcon: Icon(Icons.home),
            label: '首页',
          ),
          NavigationDestination(
            icon: Icon(Icons.history_outlined),
            selectedIcon: Icon(Icons.history),
            label: '历史',
          ),
          NavigationDestination(
            icon: Icon(Icons.settings_outlined),
            selectedIcon: Icon(Icons.settings),
            label: '设置',
          ),
        ],
      ),
    );
  }
}
