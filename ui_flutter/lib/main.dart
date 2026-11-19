import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:ui_flutter/view/home_view/home_view_page.dart';
import 'package:ui_flutter/view/home_view/home_view_bindings.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: '打印中继',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      initialBinding: HomeViewBindings(),
      home: HomeViewPage(),
    );
  }
}
