import { useDownloadDialogState } from "@/lib/state/downloads";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface DownloadRequestResponse {
  status: "OK" | "error";
  download_id: string;
}

interface DownloadProgressResponse {
  download_id: string;
  download_progress: number;
}

export default function DownloadProgress() {
  const {
    selectedDownloadables,
    selectedDataTypes,
    selectedTimeframe,
    selectedStartDate,
    selectedEndDate,
  } = useDownloadDialogState();

  const [downloadId, setDownloadId] = useState<string>();
  const [progress, setProgress] = useState<number>(0);

  useEffect(() => {
    const startDownload = async () => {
      const data = {
        downloadables: selectedDownloadables,
        dataTypes: selectedDataTypes,
        timeframe: selectedTimeframe,
        startDate: selectedStartDate,
        endDate: selectedEndDate,
      };

      // console.log(data);
      const { status, download_id }: DownloadRequestResponse = await invoke(
        "download_request",
        { data }
      );

      if (status == "OK") {
        setDownloadId(download_id);
      }
    };

    startDownload();
  }, []);

  useEffect(() => {
    if (!downloadId) return;
    const unlisten = listen("download_progress", (event) => {
      const { download_id, download_progress } =
        event.payload as DownloadProgressResponse;
      if (download_id === downloadId) {
        setProgress(download_progress);
        if (progress === 100) {
          unlisten.then((fn) => fn());
        }
      }
    });
  }, [downloadId]);

  return (
    <>
      <div className="w-full bg-base-200 flex items-center justify-center">
        <div className="p-4 px-6 w-full flex gap-2 items-center justify-center">
          <progress
            className="progress progress-primary w-full"
            value="100"
            max="100"
          ></progress>
          <div className="w-12 flex items-center justify-center">
            <kbd className="kbd">{progress}%</kbd>
          </div>
        </div>
      </div>
    </>
  );
}
