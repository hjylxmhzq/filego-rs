export default function ImagePreview({ src }: { src: string }) {
  return <div style={{ display: 'flex', justifyContent: 'center' }} >
    <img style={{ maxWidth: '100%', maxHeight: '100vh' }} src={src} alt={src} />
  </ div>
}