unit preview;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils, Dialogs, Process, ExtCtrls, Graphics;

var
  FRenderProcess: TProcess;
  FIsRendering: Boolean;

procedure SavePreviewState;
procedure LoadPreviewState;
procedure StartAsyncRender;
procedure CheckRenderStatus(AImage: TImage);

implementation

procedure StartAsyncRender;
var
  ExecutablePath: string;
begin
  if FIsRendering then Exit;
  
  if not Assigned(FRenderProcess) then
  begin
    FRenderProcess := TProcess.Create(nil);
    FRenderProcess.Options := [poNoConsole];
  end;
  
  // Try to find renderer.exe in common locations
  ExecutablePath := 'renderer.exe';
  if not FileExists(ExecutablePath) then
    ExecutablePath := 'target' + PathDelim + 'debug' + PathDelim + 'renderer.exe';
  if not FileExists(ExecutablePath) then
    ExecutablePath := '..' + PathDelim + 'target' + PathDelim + 'debug' + PathDelim + 'renderer.exe';
  if not FileExists(ExecutablePath) then
    ExecutablePath := '..' + PathDelim + '..' + PathDelim + 'target' + PathDelim + 'debug' + PathDelim + 'renderer.exe';

  if not FileExists(ExecutablePath) then
  begin
    FIsRendering := False;
    Exit;
  end;

  FRenderProcess.Executable := ExecutablePath;
  FRenderProcess.Parameters.Clear;
  FRenderProcess.Parameters.Add('--scene');
  FRenderProcess.Parameters.Add('data' + PathDelim + 'default.fwp');
  FRenderProcess.Parameters.Add('--out');
  FRenderProcess.Parameters.Add('live_render.png');

  try
    FRenderProcess.Execute;
    FIsRendering := True;
  except
    FIsRendering := False;
  end;
end;

procedure CheckRenderStatus(AImage: TImage);
begin
  if not FIsRendering then Exit;
  
  if not FRenderProcess.Running then
  begin
    FIsRendering := False;
    if FileExists('live_render.png') then
    begin
      try
        AImage.Picture.LoadFromFile('live_render.png');
      except
        // Handle file access conflicts
      end;
    end;
  end;
end;

procedure SavePreviewState;
begin
end;

procedure LoadPreviewState;
begin
end;

initialization
  FRenderProcess := nil;
  FIsRendering := False;

finalization
  if Assigned(FRenderProcess) then
  begin
    if FRenderProcess.Running then FRenderProcess.Terminate(0);
    FRenderProcess.Free;
  end;

end.
