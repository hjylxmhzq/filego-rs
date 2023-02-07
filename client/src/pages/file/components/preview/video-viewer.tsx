export default function VideoPreview({ src }: { src: string }) {
  return <div style={{ display: 'flex', justifyContent: 'center' }} >
    <video style={{ maxWidth: '100%', maxHeight: '100vh' }} controls src={src}/>
  </ div>
}