
export function imagePath(filename: string): string {
  return `${process.env.PUBLIC_URL}/images/${filename}`
}

export function errorToString(e: unknown): string {
  return typeof e === 'string' ? e : typeof e === 'object' && e !== null && typeof e.toString === 'function' ? e.toString() : String(e)
}
