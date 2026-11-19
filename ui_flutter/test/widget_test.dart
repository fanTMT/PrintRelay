// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility in the flutter_test package. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter_test/flutter_test.dart';

import 'package:ui_flutter/main.dart';

void main() {
  testWidgets('top tabs switch between home, history, and settings', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(const MyApp());

    expect(find.text('退出登录'), findsOneWidget);
    expect(find.text('Item 1'), findsNothing);

    await tester.tap(find.text('历史'));
    await tester.pumpAndSettle();
    expect(find.text('Item 1'), findsOneWidget);

    await tester.tap(find.text('设置').first);
    await tester.pumpAndSettle();
    expect(find.text('设置'), findsWidgets);
  });
}
