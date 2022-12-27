#include <iostream>
#include <optional>

#include "argagg.hpp"
#include "runner_api.h"


std::optional<GameConfig> parse_config_from_args(int argc,
                                                    char const *argv[]) {
  argagg::parser argparser{{
      {"help", {"-h", "--help"}, "shows this help message", 0},
      {"delim", {"-d", "--delim"}, "delimiter (default: ,)", 1},
      {"num", {"-n", "--num"}, "number", 1},
  }};
  argagg::parser_results args;

  try {
    args = argparser.parse(argc, argv);
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return std::nullopt;
  }

  if (args.pos.size() == 0) {
    std::cerr << "Game name wasn't provided.\n";
    return std::nullopt;
  }

  return GameConfig{.game_name = args.as<std::strig>(0)};
}

int main(int argc, char const *argv[]) {
  std::optional<GameConfig> config = parse_config_from_args(argc, argv);
  if (!config) {
    return EXIT_FAILURE;
  }

  run_game(config.value());

  return EXIT_SUCCESS;
}
