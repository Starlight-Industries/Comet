import 'dart:io';

import 'package:args/args.dart';
import 'package:chalkdart/chalk.dart';
import 'package:chalkdart/chalk_x11.dart';
const String version = '0.0.1';

ArgParser buildParser() {
  return ArgParser()
    ..addFlag(
      'help',
      abbr: 'h',
      negatable: false,
      help: 'Print this usage information.',
    )
    ..addFlag(
      'men',
      abbr: 'm',
      negatable: false,
      help: 'men',
    )
    ..addFlag(
      'verbose',
      abbr: 'V',
      negatable: false,
      help: 'Show additional command output.',
    )
    ..addFlag(
      abbr: "v",
      'version',
      negatable: false,
      help: 'Print the tool version.',
    )
    ..addFlag(
      'Hello',
      abbr: "H",
      negatable: false,
      help: 'More welcoming version of help for fun',
    )
    ..addCommand(
      "init"
      );
}

void printUsage(ArgParser argParser) {
  print(chalk.purple.bold('To use comet do [comet init] to get started!' ));
  print(argParser.usage);
  }

void main(List<String> arguments) {
  final ArgParser argParser = buildParser();
  try {
    final ArgResults results = argParser.parse(arguments,);
    bool verbose = false;

    // Process the parsed arguments.
    if (results.wasParsed('help')) {
      printUsage(argParser);
      return;
    }
    if (results.wasParsed('version')) {
      print(chalk.purple.bold("You are running Comet version: $version, Run comet --help to learn more!"));
      return;
    }
    if (results.wasParsed('verbose')) {
      verbose = true;
    }
    if (results.wasParsed('Hello')) {
      print(chalk.pink.italic.bold("Hello from starlight!"));
    }

    // Act on the arguments provided.
    //print('Positional arguments: ${results.rest}');
    if (verbose) {
      print('[VERBOSE] All arguments: ${results.arguments}');
    }
  } on FormatException catch (e) {
    // Print usage information if an invalid argument was provided.
    print("${chalk.red.bold.underline(e.message)}\n");
    printUsage(argParser);
  }
}

void init() {
for(int i = 0; i < stdout.terminalLines; i++) {
  stdout.writeln();
}
}
