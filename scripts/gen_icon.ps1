Add-Type -AssemblyName System.Drawing
$bmp = New-Object System.Drawing.Bitmap 512, 512
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.Clear([System.Drawing.Color]::FromArgb(13, 15, 18))
$brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(59, 130, 246))
$g.FillEllipse($brush, 32, 32, 448, 448)
$font = New-Object System.Drawing.Font("Arial", 220, [System.Drawing.FontStyle]::Bold)
$textBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
$sf = New-Object System.Drawing.StringFormat
$sf.Alignment = [System.Drawing.StringAlignment]::Center
$sf.LineAlignment = [System.Drawing.StringAlignment]::Center
$rect = New-Object System.Drawing.RectangleF(0, 0, 512, 512)
$g.DrawString("S", $font, $textBrush, $rect, $sf)
$bmp.Save("app-icon.png", [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose()
$bmp.Dispose()
Write-Host "Icon created successfully!"
