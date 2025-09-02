import { useSidebarState } from "@/lib/state/sidebar";
import { SelectedItemType } from "@/types";
import StaticResourceContent from "../content/StaticResourceContent";
import CompositionContent from "../content/CompositionContent";
import BacktestContent from "../content/BacktestContent";

export default function SidebarContent() {
  const { selectedItem } = useSidebarState();

  //TODO: Display download informaton and not just chart (symbol name, downloaded at, start timestamp, end timestamp, download size, download location), news, metrics

  if (selectedItem?.itemType == SelectedItemType.RawData) {
    return <StaticResourceContent />;
  }

  if (selectedItem?.itemType == SelectedItemType.Composition) {
    return <CompositionContent />;
  }

  if (selectedItem?.itemType == SelectedItemType.Backtest) {
    return <BacktestContent />;
  }
}
