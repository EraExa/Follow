unit editor;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils, Dialogs, ComCtrls, Controls, StdCtrls;

procedure SaveEditorState;
procedure LoadEditorState;
procedure LoadProject(ATree: TTreeView; AProjectFile: string);
procedure ScanDirectoryForProjects(ATree: TTreeView; ADirectory: string);
procedure LoadSourceProperties(APanel: TWinControl; ASourceFile: string);

implementation

procedure PopulateWorkspaceTree(ATree: TTreeView; AProjectFile: string);
var
  Node: TTreeNode;
begin
  ATree.Items.Clear;
  Node := ATree.Items.Add(nil, ExtractFileName(AProjectFile));
  // In a real impl, we would parse JSON here. For now, add placeholders based on default.fwp
  ATree.Items.AddChild(Node, 'Core Scene (core_scene.fws)');
  ATree.Items.AddChild(Node, 'Physics Overlay (physics.fws)');
  Node.Expanded := True;
end;

procedure LoadProject(ATree: TTreeView; AProjectFile: string);
begin
  PopulateWorkspaceTree(ATree, AProjectFile);
end;

procedure ScanDirectoryForProjects(ATree: TTreeView; ADirectory: string);
var
  SR: TSearchRec;
  RootNode, ProjectNode: TTreeNode;
begin
  ATree.Items.Clear;
  RootNode := ATree.Items.Add(nil, ADirectory);
  
  // Find projects
  if FindFirst(IncludeTrailingPathDelimiter(ADirectory) + '*.fwp', faAnyFile, SR) = 0 then
  begin
    repeat
      ProjectNode := ATree.Items.AddChild(RootNode, SR.Name);
      // For demo, add one source per project
      ATree.Items.AddChild(ProjectNode, ChangeFileExt(SR.Name, '.fws'));
    until FindNext(SR) <> 0;
    FindClose(SR);
  end;

  // Find standalone sources
  if FindFirst(IncludeTrailingPathDelimiter(ADirectory) + '*.fws', faAnyFile, SR) = 0 then
  begin
    repeat
      // If it exists in tree already (from project mock), skip? 
      // Simple impl: just add all
      ATree.Items.AddChild(RootNode, SR.Name);
    until FindNext(SR) <> 0;
    FindClose(SR);
  end;
  
  RootNode.Expanded := True;
end;

procedure LoadSourceProperties(APanel: TWinControl; ASourceFile: string);
var
  LabelName: TLabel;
  EditName: TEdit;
begin
  APanel.DisableAutoSizing;
  try
    // Clear existing controls
    while APanel.ControlCount > 0 do APanel.Controls[0].Free;
    
    LabelName := TLabel.Create(APanel);
    LabelName.Parent := APanel;
    LabelName.Caption := 'Source Name:';
    LabelName.Top := 10;
    LabelName.Left := 10;
    
    EditName := TEdit.Create(APanel);
    EditName.Parent := APanel;
    EditName.Top := 30;
    EditName.Left := 10;
    EditName.Width := APanel.Width - 20;
    EditName.Text := ExtractFileName(ASourceFile);
    
    // Additional fields like TST ID, Symbolic Expression etc would go here
  finally
    APanel.EnableAutoSizing;
  end;
end;

procedure SaveEditorState;
begin
end;

procedure LoadEditorState;
begin
end;

end.
