import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:get/get.dart';
import 'package:pdfx/pdfx.dart';

class HomeViewController extends GetxController {
  static const tabs = ['首页', '历史记录', '打印机状态管理', '关于我们'];

  final selectedIndex = 0.obs;
  final showLogin = false.obs;
  final selectedFile = Rxn<PlatformFile>();
  final fileBytes = Rxn<Uint8List>();
  final pdfController = Rxn<PdfControllerPinch>();

  Future<void> pickFile() async {
    final file = await FilePicker.pickFile(
      type: FileType.custom,
      allowedExtensions: const [
        'pdf',
        'doc',
        'docx',
        'txt',
        'md',
        'png',
        'jpg',
        'jpeg',
        'gif',
        'webp',
      ],
    );
    if (file == null) return;

    final bytes = await file.readAsBytes();
    pdfController.value?.dispose();
    selectedFile.value = file;
    fileBytes.value = bytes;
    if (file.extension?.toLowerCase() == 'pdf') {
      pdfController.value = PdfControllerPinch(
        document: PdfDocument.openData(bytes),
      );
    } else {
      pdfController.value = null;
    }
  }

  void clearFile() {
    pdfController.value?.dispose();
    pdfController.value = null;
    selectedFile.value = null;
    fileBytes.value = null;
  }

  void showUploadUnavailable() {
    Get.snackbar(
      '上传暂不可用',
      '服务端尚未提供文件上传接口。',
      snackPosition: SnackPosition.BOTTOM,
    );
  }

  @override
  void onClose() {
    pdfController.value?.dispose();
    super.onClose();
  }
}
