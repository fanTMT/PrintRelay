import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:ui_flutter/view/home_view/home_view_page.dart';
import 'package:ui_flutter/view/home_view/home_view_bindings.dart';

void main() {
  const isRunningWithWasm = bool.fromEnvironment('dart.tool.dart2wasm');
  if (isRunningWithWasm) {
    debugPrint("是Wasm");
  } else {
    debugPrint("是html");
  }
  // debugPrint("是不是Wasm{$isRunningWithWasm}");
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: '打印中继',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xff5965e8)),
      ),
      initialBinding: HomeViewBindings(),
      home: HomeViewPage(),
    );
  }
}
