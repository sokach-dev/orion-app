
export function getCustomDate(offset: number) {
    const date = new Date();
    date.setDate(date.getDate() + offset);
    const customDate = date.toISOString().slice(0, 10) + ' 00:00:00';
    return customDate;
}