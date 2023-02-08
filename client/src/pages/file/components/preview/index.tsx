import mime from 'mime';
import { create_download_link, FileStat } from '../../../../apis/file';
import ImagePreview from './image-viewer';
import TextPreview from './text-viewer';
import VideoPreview from './video-viewer';

export default function Preview({ file, dir }: { file: FileStat, dir: string }) {
  let guess = mime.getType(file.name);
  let inner;
  if (guess?.includes('image')) {
    inner = <ImagePreview src={create_download_link(dir, file.name)} />
  } else if (guess?.includes('text') && file.size < 1024 * 1024 * 1024) {
    inner = <TextPreview dir={dir} file={file} />
  } else if (guess?.includes('video')) {
    inner = <VideoPreview src={create_download_link(dir, file.name)} />
  } else {
    inner = <div></div>
  }
  return <div style={{ border: '1px solid #ccc', minHeight: 200 }}>{inner}</div>
}