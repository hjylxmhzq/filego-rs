import mime from 'mime';
import { create_download_link, FileStat } from '../../../../apis/file';
import ImagePreview from './image-viewer';
import TextPreview from './text-viewer';
import VideoPreview from './video-viewer';

export default function Preview({ file, dir }: { file: FileStat, dir: string }) {
  let guess = mime.getType(file.name);
  if (guess?.includes('image')) {
    return <ImagePreview src={create_download_link(dir, file.name)} />
  }
  if (guess?.includes('text') && file.size < 1024 * 1024 * 1024) {
    return <TextPreview dir={dir} file={file} />
  }
  if (guess?.includes('video')) {
    return <VideoPreview src={create_download_link(dir, file.name)} />
  }
  return <div>111</div>
}