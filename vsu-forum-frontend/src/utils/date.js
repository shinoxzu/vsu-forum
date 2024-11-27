export function formatDate(dateString) {
    const date = new Date(dateString);
    return new Intl.DateTimeFormat("ru-RU", {
        year: "numeric",
        month: "long",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
    }).format(date);
}

export function formatRelativeTime(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diff = now - date;
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) {
        return formatDate(dateString);
    } else if (hours > 0) {
        return `${hours} ${pluralize(hours, "час", "часа", "часов")} назад`;
    } else if (minutes > 0) {
        return `${minutes} ${pluralize(minutes, "минуту", "минуты", "минут")} назад`;
    } else {
        return "только что";
    }
}

function pluralize(number, one, two, five) {
    let n = Math.abs(number);
    n %= 100;
    if (n >= 5 && n <= 20) {
        return five;
    }
    n %= 10;
    if (n === 1) {
        return one;
    }
    if (n >= 2 && n <= 4) {
        return two;
    }
    return five;
}
