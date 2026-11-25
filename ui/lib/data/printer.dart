class PrinterInfo {
  const PrinterInfo({
    required this.name,
    required this.state,
    required this.isPrinting,
    required this.isDefault,
  });

  final String name;
  final String state;
  final bool isPrinting;
  final bool isDefault;

  factory PrinterInfo.fromJson(Map<String, dynamic> json) {
    return PrinterInfo(
      name: json['name'] as String? ?? '未命名打印机',
      state: json['state'] as String? ?? 'unknown',
      isPrinting: json['is_printing'] as bool? ?? false,
      isDefault: json['is_def'] as bool? ?? false,
    );
  }

  String get statusLabel {
    if (isPrinting || state.toLowerCase().contains('printing')) return '正在打印';
    if (state.toLowerCase().contains('paused')) return '已暂停';
    if (state.toLowerCase().contains('disabled') ||
        state.toLowerCase().contains('error')) {
      return '错误';
    }
    if (state.toLowerCase().contains('offline')) return '离线';
    return '空闲';
  }
}
