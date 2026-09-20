#include "error.hpp"

#include <system_error>
#include <utility>

#include "singleton.hpp"

namespace cha {

namespace {

class IoErrorCategory final
    : public std::error_category
    , public Singleton<IoErrorCategory> {
public:
    [[nodiscard]] auto name() const noexcept -> const char* override { return "io"; }

    [[nodiscard]] auto message(int ev) const -> std::string override {
        switch (static_cast<IoError>(ev)) {
            case IoError::MissingArgument: return "Missing argument";
        }
        std::unreachable();
    }
};

}    // namespace

auto make_error_code(IoError e) noexcept -> std::error_code {
    return {static_cast<int>(e), IoErrorCategory::instance()};
}

}    // namespace cha
