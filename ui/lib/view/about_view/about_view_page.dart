import 'package:flutter/material.dart';

class AboutViewPage extends StatelessWidget {
  const AboutViewPage({super.key});

  @override
  Widget build(BuildContext context) {
    final textStyle = Theme.of(context).textTheme.bodyLarge
        ?.copyWith(height: 1.8, color: const Color(0xff647084));
    return ListView(
      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 40),
      children: [
        Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 900),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.symmetric(
                    horizontal: 32,
                    vertical: 38,
                  ),
                  decoration: BoxDecoration(
                    color: const Color(0xfff0f2ff),
                    borderRadius: BorderRadius.circular(16),
                  ),
                  child: Column(
                    children: [
                      Text(
                        '关于我们',
                        style: Theme.of(context).textTheme.titleSmall?.copyWith(
                          color: Theme.of(context).colorScheme.primary,
                          fontWeight: FontWeight.w700,
                        ),
                      ),
                      const SizedBox(height: 10),
                      Text(
                        '用心创造更好的体验',
                        textAlign: TextAlign.center,
                        style: Theme.of(context).textTheme.headlineMedium
                            ?.copyWith(fontWeight: FontWeight.w700),
                      ),
                      const SizedBox(height: 12),
                      Text(
                        '我们专注于打造简单、实用且可靠的产品，帮助每一位用户更轻松地实现自己的目标。',
                        textAlign: TextAlign.center,
                        style: textStyle,
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 36),
                LayoutBuilder(
                  builder: (context, constraints) {
                    final narrow = constraints.maxWidth < 620;
                    final sections = [
                      _AboutSection(
                        title: '我们的使命',
                        body: '以用户需求为出发点，持续探索更好的解决方案。我们相信，优秀的产品源于倾听、协作与不断改进。',
                        style: textStyle,
                      ),
                      _AboutSection(
                        title: '我们的价值观',
                        body: '以人为本，关注真实需求；持续改进，保持好奇；真诚协作，共同创造长期价值。',
                        style: textStyle,
                      ),
                    ];
                    return narrow
                        ? Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: sections,
                          )
                        : Row(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: sections
                                .map((section) => Expanded(child: section))
                                .toList(),
                          );
                  },
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _AboutSection extends StatelessWidget {
  const _AboutSection({required this.title, required this.body, this.style});

  final String title;
  final String body;
  final TextStyle? style;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(right: 28, bottom: 24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(title, style: Theme.of(context).textTheme.titleLarge),
          const SizedBox(height: 10),
          Text(body, style: style),
        ],
      ),
    );
  }
}
