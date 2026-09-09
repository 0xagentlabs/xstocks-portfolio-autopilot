import "./globals.css";
import Providers from "../components/Providers";
export const metadata = { title: "xStocks Autopilot", description: "Non-custodial xStocks portfolio automation on Solana devnet" };
export default function Layout({children}:{children:React.ReactNode}) { return <html lang="zh-CN"><body><Providers>{children}</Providers></body></html>; }

