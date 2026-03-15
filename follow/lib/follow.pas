unit follow;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, ComCtrls, StdCtrls, Menus,
  ExtCtrls, editor, preview, prompt;

type

  { TForm1 }

  TForm1 = class(TForm)
    LeftBox: TGroupBox;
    RightBox: TGroupBox;
    MainGroup: TPageControl;
    TabSheetEditor: TTabSheet;
    TabSheetPreview: TTabSheet;
    PreviewImage: TImage;
    Console: TGroupBox;
    WorkspaceTree: TTreeView;
    PropScrollBox: TScrollBox;
    ToolBar1: TToolBar;
    RenderTimer: TTimer;
    MainMenu1: TMainMenu;
    MenuItemFile: TMenuItem;
    MenuItemEdit: TMenuItem;
    MenuItemView: TMenuItem;
    MenuItemToggleEditor: TMenuItem;
    MenuItemTogglePreview: TMenuItem;
    SplitterLeft: TSplitter;
    SplitterRight: TSplitter;
    SplitterBottom: TSplitter;
    procedure FormShow(Sender: TObject);
    procedure MainGroupChange(Sender: TObject);
    procedure MenuItemToggleEditorClick(Sender: TObject);
    procedure MenuItemTogglePreviewClick(Sender: TObject);
    procedure RenderTimerTimer(Sender: TObject);
    procedure WorkspaceTreeChange(Sender: TObject; Node: TTreeNode);
  private
    FCurrentView: string;
    procedure UpdateSidePanels;
    procedure SaveViewState(AView: string);
    procedure LoadViewState(AView: string);
  public

  end;

var
  Form1: TForm1;

implementation

{$R *.lfm}

{ TForm1 }

procedure TForm1.FormShow(Sender: TObject);
begin
  // Show prompt form right after main form loaded
  if PromptForm.ShowModal = mrYes then
  begin
    if PromptForm.SelectedPath <> '' then
      ScanDirectoryForProjects(WorkspaceTree, PromptForm.SelectedPath)
    else
      LoadProject(WorkspaceTree, 'data/default.fwp');
  end
  else
  begin
     // Create New or Cancelled
     LoadProject(WorkspaceTree, 'data/template.fwp');
  end;
  
  FCurrentView := MainGroup.ActivePage.Name;
  LoadViewState(FCurrentView);
  UpdateSidePanels;
end;

procedure TForm1.MainGroupChange(Sender: TObject);
var
  NewView: string;
begin
  NewView := MainGroup.ActivePage.Name;
  if FCurrentView <> NewView then
  begin
    SaveViewState(FCurrentView);
    FCurrentView := NewView;
    LoadViewState(FCurrentView);
  end;
  UpdateSidePanels;
end;

procedure TForm1.SaveViewState(AView: string);
begin
  if AView = 'TabSheetEditor' then SaveEditorState
  else if AView = 'TabSheetPreview' then SavePreviewState;
end;

procedure TForm1.LoadViewState(AView: string);
begin
  if AView = 'TabSheetEditor' then
  begin
     LoadEditorState;
     RenderTimer.Enabled := False;
  end
  else if AView = 'TabSheetPreview' then
  begin
     LoadPreviewState;
     RenderTimer.Enabled := True;
  end;
end;

procedure TForm1.MenuItemToggleEditorClick(Sender: TObject);
begin
  TabSheetEditor.TabVisible := MenuItemToggleEditor.Checked;
end;

procedure TForm1.MenuItemTogglePreviewClick(Sender: TObject);
begin
  TabSheetPreview.TabVisible := MenuItemTogglePreview.Checked;
end;

procedure TForm1.UpdateSidePanels;
begin
  if MainGroup.ActivePage = TabSheetEditor then
  begin
    LeftBox.Caption := 'Editor Tools';
    RightBox.Caption := 'Field Properties';
  end
  else if MainGroup.ActivePage = TabSheetPreview then
  begin
    LeftBox.Caption := 'Scene Hierarchy';
    RightBox.Caption := 'Render Settings';
  end;
end;

procedure TForm1.RenderTimerTimer(Sender: TObject);
begin
  if MainGroup.ActivePage = TabSheetPreview then
  begin
    CheckRenderStatus(PreviewImage);
    StartAsyncRender;
  end;
end;

procedure TForm1.WorkspaceTreeChange(Sender: TObject; Node: TTreeNode);
begin
  if Assigned(Node) and (Node.Level > 0) then
  begin
    // Extract file name from node text for demo
    LoadSourceProperties(PropScrollBox, Node.Text);
  end;
end;

end.

