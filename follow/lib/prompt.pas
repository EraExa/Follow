unit prompt;

{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils, Forms, Controls, Graphics, Dialogs, StdCtrls;

type

  { TPromptForm }

  TPromptForm = class(TForm)
    ButtonNew: TButton;
    ButtonLoad: TButton;
    LabelTitle: TLabel;
    SelectDirectoryDialog: TSelectDirectoryDialog;
    procedure ButtonNewClick(Sender: TObject);
    procedure ButtonLoadClick(Sender: TObject);
  private

  public
    SelectedPath: string;
  end;

var
  PromptForm: TPromptForm;

implementation

{$R *.lfm}

{ TPromptForm }

procedure TPromptForm.ButtonNewClick(Sender: TObject);
begin
  ModalResult := mrOk;
end;

procedure TPromptForm.ButtonLoadClick(Sender: TObject);
begin
  if SelectDirectoryDialog.Execute then
  begin
    SelectedPath := SelectDirectoryDialog.FileName;
    ModalResult := mrYes;
  end;
end;

end.
