#pragma once

#include <expected>
#include <format>
#include <system_error>

namespace cha {

template <typename T>
using Result = std::expected<T, std::error_code>;

}    // namespace cha

template <>
struct std::formatter<std::error_category> : std::formatter<std::string_view> {
    template <typename Ctx>
    auto format(const std::error_category& cat, Ctx& ctx) const {
        return std::formatter<std::string_view>::format(cat.name(), ctx);
    }
};

template <>
struct std::formatter<std::error_code> : std::formatter<std::string_view> {
    template <typename Ctx>
    auto format(const std::error_code& ec, Ctx& ctx) const {
        return std::formatter<std::string_view>::format(
            std::format("{}: {} ({})", ec.category().name(), ec.message(), ec.value()), ctx);
    }
};

namespace cha {

// NOLINTNEXTLINE(performance-enum-size) - Casted by standard anyways
enum class IoError : int { MissingArgument };

auto make_error_code(IoError e) noexcept -> std::error_code;

}    // namespace cha

template <>
struct std::is_error_code_enum<cha::IoError> : true_type {};
