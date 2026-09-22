import { invokeCommand } from './ipc';
import type { IngestedItemDTO, IngestFilterDTO } from '../types/ingest';

export async function ingestFile(filePath: string): Promise<IngestedItemDTO> {
  return invokeCommand<IngestedItemDTO>('ingest_file', { filePath });
}

export async function ingestFileContent(fileName: string, content: string): Promise<IngestedItemDTO> {
  return invokeCommand<IngestedItemDTO>('ingest_file_content', { fileName, content });
}

export async function ingestUrl(url: string): Promise<IngestedItemDTO> {
  return invokeCommand<IngestedItemDTO>('ingest_url', { url });
}

export async function listIngestedItems(filter?: IngestFilterDTO): Promise<IngestedItemDTO[]> {
  return invokeCommand<IngestedItemDTO[]>('list_ingested_items', { filter });
}
