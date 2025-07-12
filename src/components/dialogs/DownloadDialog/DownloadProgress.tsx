import { useDownloadDialogState } from "@/lib/state/downloads";
import { useEffect, useState } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
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
    selectedDataTypes,
    selectedTimeframe,
    selectedStartDate,
    selectedEndDate,
    availableSelectedDownloadables
  } = useDownloadDialogState();

  const [downloadId, setDownloadId] = useState<string>();
  const [progress, setProgress] = useState<number>(0);

  useEffect(() => {
    let progressUnlisten: (() => void) | null = null;

    const startDownload = async () => {
      try {
        progressUnlisten = await listen<DownloadProgressResponse>(
          "download_progress",
          (event) => {
            const { download_progress } = event.payload;

            setProgress(download_progress);

            if (download_progress === 100) {
              if (progressUnlisten) {
                progressUnlisten();
                progressUnlisten = null;
              }
            }
          }
        );

        const data = {
          downloadables: availableSelectedDownloadables,
          dataTypes: selectedDataTypes,
          timeframe: selectedTimeframe,
          startDate: selectedStartDate,
          endDate: selectedEndDate,
        };

        const { status, download_id }: DownloadRequestResponse = await invoke(
          "download_request",
          { data }
        );

        if (status === "OK") {
          setDownloadId(download_id);
        }
      } catch (error) {
        console.error("Failed to start download:", error);
      }
    };

    startDownload();

    return () => {
      if (progressUnlisten) {
        progressUnlisten();
      }
    };
  }, []);

  return (
    <>
      <div className="w-full bg-base-200 flex items-center justify-center">
        <div className="p-4 px-6 w-full flex gap-2 items-center justify-center">
          <progress
            className="progress progress-primary w-full"
            value={progress.toFixed(1)}
            max={100}
          ></progress>
          <div className="w-12 flex items-center justify-center">
            <kbd className="kbd p-1">{progress.toFixed(1)}%</kbd>
          </div>
        </div>
      </div>
    </>
  );
}
