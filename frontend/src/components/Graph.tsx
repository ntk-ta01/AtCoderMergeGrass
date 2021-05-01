import CalendarHeatmap from "./CalendarHeatmap";

const COLORS_COUNT = ["#ebedf0", "#c6e48b", "#7bc96f", "#239a3b", "#196127"];
const WEEKDAY = 7;
const WEEKS = 53;

const formatCountTooltip = (
    date: string,
    count: number
): string => {
    if (count === 1) {
        return `${date} ${count} contribution`;
    } else {
        return `${date} ${count} contributions`;
    }
};

interface Props {
    dates: string[],
    values: number[],
}

export default function Graph(props: Props) {
    const values = props.values;
    const dates = props.dates;

    const formatTooltip = (date: string, count: number): string => formatCountTooltip(date, count);
    const createTableData = (
        dates: string[],
        values: number[],
    ): { date: string; value?: number }[] => {
        const tableData: { date: string; value?: number }[] = [];
        for (let i = 0; i < dates.length; i++) {
            const date = dates[i];
            const value = values[i];
            tableData.push({ date, value });
        }
        return tableData;
    };
    const tableData = createTableData(
        dates,
        values
    );
    const getColor = (_: string, count: number): string =>
        COLORS_COUNT[Math.min(count, COLORS_COUNT.length - 1)];
    return (
        <CalendarHeatmap
            tableData={tableData}
            formatTooltip={formatTooltip}
            getColor={getColor}
            columns={WEEKS}
            rows={WEEKDAY}
        />
    );
}