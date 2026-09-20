#include <cstdlib>
#include <span>

#include "error.hpp"
#include "logger.hpp"

namespace cha {

namespace {

struct Cli {
    std::string_view input_path;
};

auto parseCli(std::span<char*> args) -> Result<Cli> {
    if (args.size() != 2) { return std::unexpected(make_error_code(IoError::MissingArgument)); }
    return {Cli{.input_path = args[1]}};
}

}    // namespace

}    // namespace cha

auto main(int argc, char* argv[]) -> int {
    cha::log::set_level(cha::log::Level::Trace);
    auto cli_res = cha::parseCli(std::span(argv, argc));
    if (!cli_res) {
        cha::log::error("Failed to parse cli: {}", cli_res.error());
        return EXIT_FAILURE;
    }
    auto cli = std::move(cli_res).value();

    cha::log::debug("Running with input path: {}", cli.input_path);
}
