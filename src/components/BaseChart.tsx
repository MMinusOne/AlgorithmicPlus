export default function BaseChart() {
  return <></>;
}
/**
 *   useEffect(() => {
     if (!chartContainer.current) return;
 
     const chart = createChart(chartContainer.current, {
       width: chartContainer.current.clientWidth,
       height: chartContainer.current.clientHeight,
       layout: { attributionLogo: false },
     });
 
     const updateSize = () => {
       if (!chartContainer.current) return;
 
       chart.applyOptions({
         width: chartContainer.current.clientWidth,
         height: chartContainer.current.clientHeight,
       });
     };
 
     const resizeObserver = new ResizeObserver(() => {
       updateSize();
     });
 
     resizeObserver.observe(chartContainer.current);
 
     chartContainer.current.addEventListener("resize", () => {
       updateSize();
     });
 
     window.addEventListener("resize", () => {
       updateSize();
     });
 
     const lineSeries = chart.addSeries(LineSeries);
 
     lineSeries.setData([
       { time: "2018-12-12", value: 24.11 },
       { time: "2018-12-13", value: 31.74 },
       { time: "2018-12-14", value: 31.74 },
       { time: "2018-12-15", value: 31.74 },
       { time: "2018-12-16", value: 31.74 },
       { time: "2018-12-17", value: 31.74 },
       { time: "2018-12-18", value: 31.74 },
       { time: "2018-12-19", value: 31.74 },
       { time: "2018-12-20", value: 31.74 },
       { time: "2018-12-21", value: 31.74 },
       { time: "2018-12-22", value: 31.74 },
       { time: "2018-12-23", value: 31.74 },
       { time: "2018-12-24", value: 31.74 },
       { time: "2018-12-25", value: 31.74 },
       { time: "2018-12-26", value: 31.74 },
       { time: "2018-12-27", value: 31.74 },
       { time: "2018-12-28", value: 31.74 },
       { time: "2018-12-29", value: 31.74 },
       { time: "2018-12-30", value: 31.74 },
       { time: "2019-01-01", value: 31.74 },
       { time: "2019-01-02", value: 31.74 },
       { time: "2019-01-03", value: 31.74 },
       { time: "2019-01-04", value: 31.74 },
     ]);
 
     return () => {
       resizeObserver.disconnect();
       window.removeEventListener("resize", updateSize);
       chart.remove();
     };
   }, [chartContainer]);
 
 */