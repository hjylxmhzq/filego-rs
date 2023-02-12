import mime from 'mime';
import { create_download_link, FileStat } from '../../../../apis/file';
import ImagePreview from './image-viewer';
import TextPreview from './text-viewer';
import VideoPreview from './video-viewer';
import style from './index.module.less';
import { useCallback, useState } from 'react';

export default function Preview({ files, dir, file, onClose }: { files: FileStat[], dir: string, file: FileStat, onClose?: () => void }) {
  const [title, setTitle] = useState(file.name);

  const onPreviewingChange = useCallback((f: FileStat) => { setTitle(f?.name || '') }, []);

  let guess = mime.getType(file.name);
  let inner;
  if (guess?.includes('image')) {
    inner = <ImagePreview dir={dir} files={files} file={file} onPreviewingChange={onPreviewingChange} />
  } else if (guess?.includes('text') && file.size < 1024 * 1024 * 1024) {
    inner = <TextPreview dir={dir} file={file} />
  } else if (guess?.includes('video')) {
    inner = <VideoPreview src={create_download_link(dir, file.name)} />
  } else {
    inner = <div></div>
  }
  return <div className={style['preview']}>
    <div className={style['preview-title-bar']}><span style={{ height: 25, lineHeight: '25px' }}>{title}</span><span style={{ cursor: 'pointer' }} onClick={() => onClose?.()}>X</span></div>
    <div style={{ minHeight: 200 }}>{inner}</div>
  </div>
}