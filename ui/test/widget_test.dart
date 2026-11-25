// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility in the flutter_test package. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'dart:ui' as ui;

import 'package:flutter_test/flutter_test.dart';
import 'package:get/get.dart';

import 'package:ui_flutter/main.dart';

void main() {
  testWidgets('main navigation and login entry work', (
    WidgetTester tester,
  ) async {
    Get.reset();
    await tester.pumpWidget(const MyApp());

    expect(find.text('共享打印服务'), findsOneWidget);
    expect(find.text('未登录 · 登录'), findsOneWidget);
    expect(find.text('文件预览'), findsOneWidget);
    expect(find.text('选择本地文件'), findsOneWidget);

    await tester.tap(find.text('关于我们'));
    await tester.pumpAndSettle();
    expect(find.text('用心创造更好的体验'), findsOneWidget);

    await tester.tap(find.text('未登录 · 登录'));
    await tester.pumpAndSettle();
    expect(find.text('欢迎回来'), findsOneWidget);
    expect(find.text('登录服务暂未开放'), findsOneWidget);

    await tester.tap(find.byTooltip('返回'));
    await tester.pumpAndSettle();
    expect(find.text('共享打印服务'), findsOneWidget);
  });

  testWidgets('mobile starts on file preview with bottom navigation', (
    WidgetTester tester,
  ) async {
    Get.reset();
    tester.view.physicalSize = const ui.Size(390, 844);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await tester.pumpWidget(const MyApp());

    expect(find.text('文件预览'), findsOneWidget);
    expect(find.text('选择本地文件'), findsOneWidget);
    expect(find.text('打印机'), findsOneWidget);
  });
}
